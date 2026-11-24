use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode, Operand, SIMDSizeCode, SizeCode};

use super::super::helpers::{do_writeback, guard_mem, resolve_mem_addr, write_fp, write_reg};

fn simd_bytes(size: SIMDSizeCode) -> usize {
    (elem_bits(size) / 8) as usize
}

pub fn transfer_structure(emu: &mut Emu, ins: &Instruction) -> bool {
    let (first, count, element, lanes, first_lane) = match ins.operands[0] {
        Operand::SIMDRegisterGroup(size, reg, element, count) => {
            let width = simd_bytes(element);
            (reg, count as usize, width, simd_bytes(size) / width, None)
        }
        Operand::SIMDRegisterGroupLane(reg, size, count, lane) => (
            reg,
            count as usize,
            simd_bytes(size),
            1,
            Some(lane as usize),
        ),
        _ => return false,
    };
    let (addr, writeback) = resolve_mem_addr(emu, &ins.operands[1]);
    let length = count * lanes * element;
    let store = matches!(
        ins.opcode,
        Opcode::ST1 | Opcode::ST2 | Opcode::ST3 | Opcode::ST4
    );
    let interleaved = !matches!(ins.opcode, Opcode::LD1 | Opcode::ST1);
    guard_mem(emu, addr, length as u32, store);
    let mut bytes = [0; 64];
    if !store {
        let Some(data) = emu.maps.try_read_bytes(addr, length) else {
            return false;
        };
        bytes[..length].copy_from_slice(&data);
    }
    for index in 0..count {
        let reg = (first as usize + index) % 32;
        let mut vector = if store || first_lane.is_some() {
            emu.regs_aarch64().v[reg].to_le_bytes()
        } else {
            [0; 16]
        };
        for lane in 0..lanes {
            let memory = if interleaved {
                lane * count + index
            } else {
                index * lanes + lane
            } * element;
            let offset = (first_lane.unwrap_or(0) + lane) * element;
            let memory = &mut bytes[memory..memory + element];
            let value = &mut vector[offset..offset + element];
            if store {
                memory.copy_from_slice(value);
            } else {
                value.copy_from_slice(memory);
            }
        }
        if !store {
            emu.regs_aarch64_mut().v[reg] = u128::from_le_bytes(vector);
        }
    }
    if store && !emu.maps.write_bytes(addr, &bytes[..length]) {
        return false;
    }
    do_writeback(emu, writeback);
    true
}

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    match ins.opcode {
        Opcode::MOVI | Opcode::MVNI => vector_immediate(emu, ins),
        Opcode::FMOV => exec_fmov(emu, ins),
        Opcode::DUP => exec_dup(emu, ins),
        Opcode::UMOV => exec_umov(emu, ins),
        Opcode::INS => exec_ins(emu, ins),
        Opcode::NOT => exec_not(emu, ins),
        Opcode::ORR
        | Opcode::BIC
        | Opcode::AND
        | Opcode::EOR
        | Opcode::ORN
        | Opcode::BSL
        | Opcode::BIT
        | Opcode::BIF => vector_bitwise(emu, ins),
        Opcode::ADD | Opcode::SUB | Opcode::CMEQ | Opcode::CMHI | Opcode::CMHS => {
            integer_lanes(emu, ins)
        }
        Opcode::UMAXP => pairwise_unsigned_max(emu, ins),
        Opcode::EXT => extract_vector(emu, ins),
        Opcode::SHL | Opcode::USHR => shift_lanes(emu, ins),
        Opcode::SHLL
        | Opcode::SHLL2
        | Opcode::USHLL
        | Opcode::USHLL2
        | Opcode::SSHLL
        | Opcode::SSHLL2 => widen_lanes(emu, ins),
        Opcode::ADDHN
        | Opcode::ADDHN2
        | Opcode::SHRN
        | Opcode::SHRN2
        | Opcode::XTN
        | Opcode::XTN2 => narrow_lanes(emu, ins),
        Opcode::ZIP1 | Opcode::ZIP2 => zip(emu, ins),
        _ => {
            log::warn!("simd: unhandled opcode {:?}", ins.opcode);
            true
        }
    }
}

struct Vector {
    reg: usize,
    bits: u32,
    lane: u32,
}

impl Vector {
    fn decode(op: Operand) -> Option<Self> {
        let (size, reg, lane) = match op {
            Operand::SIMDRegisterElements(size, reg, lane) => (size, reg, lane),
            Operand::SIMDRegister(size, reg) => (size, reg, size),
            _ => return None,
        };
        Some(Self {
            reg: reg as usize,
            bits: elem_bits(size),
            lane: elem_bits(lane),
        })
    }
}

fn vector_bitwise(emu: &mut Emu, ins: &Instruction) -> bool {
    if matches!(ins.operands[2], Operand::Nothing) {
        return vector_immediate(emu, ins);
    }
    let [Some(dst), Some(left), Some(right)] =
        [ins.operands[0], ins.operands[1], ins.operands[2]].map(Vector::decode)
    else {
        return false;
    };
    let regs = emu.regs_aarch64();
    let lhs = regs.v[left.reg];
    let rhs = regs.v[right.reg];
    let value = match ins.opcode {
        Opcode::ORR => lhs | rhs,
        Opcode::BIC => lhs & !rhs,
        Opcode::AND => lhs & rhs,
        Opcode::EOR => lhs ^ rhs,
        Opcode::ORN => lhs | !rhs,
        Opcode::BSL => (lhs & regs.v[dst.reg]) | (rhs & !regs.v[dst.reg]),
        Opcode::BIT => (lhs & rhs) | (regs.v[dst.reg] & !rhs),
        Opcode::BIF => (lhs & !rhs) | (regs.v[dst.reg] & rhs),
        _ => return false,
    } & lane_mask(dst.bits);
    emu.regs_aarch64_mut().v[dst.reg] = value;
    true
}

fn integer_lanes(emu: &mut Emu, ins: &Instruction) -> bool {
    let (Some(dst), Some(src)) = (
        Vector::decode(ins.operands[0]),
        Vector::decode(ins.operands[1]),
    ) else {
        return false;
    };
    let regs = emu.regs_aarch64();
    let lhs = regs.v[src.reg];
    let rhs = match Vector::decode(ins.operands[2]) {
        Some(reg) => regs.v[reg.reg],
        None if matches!(ins.operands[2], Operand::Imm64(0)) && ins.opcode == Opcode::CMEQ => 0,
        None => return false,
    };
    let mask = lane_mask(dst.lane);
    let mut result = 0;
    for i in 0..dst.bits / dst.lane {
        let shift = i * dst.lane;
        let a = (lhs >> shift) & mask;
        let b = (rhs >> shift) & mask;
        let value = match ins.opcode {
            Opcode::ADD => a.wrapping_add(b) & mask,
            Opcode::SUB => a.wrapping_sub(b) & mask,
            Opcode::CMEQ if a == b => mask,
            Opcode::CMHI if a > b => mask,
            Opcode::CMHS if a >= b => mask,
            Opcode::CMEQ | Opcode::CMHI | Opcode::CMHS => 0,
            _ => return false,
        };
        result |= value << shift;
    }
    emu.regs_aarch64_mut().v[dst.reg] = result;
    true
}

fn pairwise_unsigned_max(emu: &mut Emu, ins: &Instruction) -> bool {
    let [Some(dst), Some(left), Some(right)] =
        [ins.operands[0], ins.operands[1], ins.operands[2]].map(Vector::decode)
    else {
        return false;
    };
    let regs = emu.regs_aarch64();
    let pairs = dst.bits / (2 * dst.lane);
    let mask = lane_mask(dst.lane);
    let mut result = 0;
    for (half, reg) in [left.reg, right.reg].into_iter().enumerate() {
        let source = regs.v[reg];
        for i in 0..pairs {
            let pair = source >> (2 * i * dst.lane);
            let maximum = (pair & mask).max((pair >> dst.lane) & mask);
            result |= maximum << ((half as u32 * pairs + i) * dst.lane);
        }
    }
    emu.regs_aarch64_mut().v[dst.reg] = result;
    true
}

fn shift_lanes(emu: &mut Emu, ins: &Instruction) -> bool {
    let (Some(dst), Some(src)) = (
        Vector::decode(ins.operands[0]),
        Vector::decode(ins.operands[1]),
    ) else {
        return false;
    };
    let shift = match ins.operands[2] {
        Operand::Imm16(n) => n as u32,
        Operand::Immediate(n) => n,
        _ => return false,
    };
    let value = emu.regs_aarch64().v[src.reg];
    let mask = lane_mask(dst.lane);
    let mut result = 0;
    for i in 0..dst.bits / dst.lane {
        let pos = i * dst.lane;
        let lane = (value >> pos) & mask;
        let shifted = if ins.opcode == Opcode::USHR {
            lane >> shift
        } else {
            (lane << shift) & mask
        };
        result |= shifted << pos;
    }
    emu.regs_aarch64_mut().v[dst.reg] = result;
    true
}

fn narrow_lanes(emu: &mut Emu, ins: &Instruction) -> bool {
    let (Some(dst), Some(src)) = (
        Vector::decode(ins.operands[0]),
        Vector::decode(ins.operands[1]),
    ) else {
        return false;
    };
    let regs = emu.regs_aarch64();
    let lhs = regs.v[src.reg];
    let (rhs, shift) = match ins.operands[2] {
        Operand::Nothing => (0, 0),      // XTN
        Operand::Immediate(n) => (0, n), // SHRN
        Operand::Imm16(n) => (0, n as u32),
        operand => {
            let Some(other) = Vector::decode(operand) else {
                return false;
            };
            (regs.v[other.reg], dst.lane) // ADDHN takes the high half of each sum.
        }
    };
    let mut result = 0;
    for i in 0..64 / dst.lane {
        let pos = i * src.lane;
        let a = (lhs >> pos) & lane_mask(src.lane);
        let b = (rhs >> pos) & lane_mask(src.lane);
        result |= (((a + b) >> shift) & lane_mask(dst.lane)) << (i * dst.lane);
    }
    if dst.bits == 128 {
        result = (result << 64) | (regs.v[dst.reg] & lane_mask(64));
    }
    emu.regs_aarch64_mut().v[dst.reg] = result;
    true
}

fn widen_lanes(emu: &mut Emu, ins: &Instruction) -> bool {
    let (Some(dst), Some(src)) = (
        Vector::decode(ins.operands[0]),
        Vector::decode(ins.operands[1]),
    ) else {
        return false;
    };
    let regs = emu.regs_aarch64();
    let source = if matches!(ins.opcode, Opcode::SHLL2 | Opcode::SSHLL2 | Opcode::USHLL2) {
        regs.v[src.reg] >> 64
    } else {
        regs.v[src.reg]
    };
    let shift = match ins.opcode {
        Opcode::SHLL | Opcode::SHLL2 => src.lane,
        _ => match ins.operands[2] {
            Operand::Immediate(n) => n,
            Operand::Imm16(n) => n as u32,
            _ => return false,
        },
    };
    let mut result = 0;
    for i in 0..128 / dst.lane {
        let lane = (source >> (i * src.lane)) & lane_mask(src.lane);
        let signed = matches!(ins.opcode, Opcode::SSHLL | Opcode::SSHLL2);
        let lane = if signed && lane & (1 << (src.lane - 1)) != 0 {
            lane | !lane_mask(src.lane)
        } else {
            lane
        };
        let lane = (lane << shift) & lane_mask(dst.lane);
        result |= lane << (i * dst.lane);
    }
    emu.regs_aarch64_mut().v[dst.reg] = result;
    true
}

fn extract_vector(emu: &mut Emu, ins: &Instruction) -> bool {
    let [Some(dst), Some(left), Some(right)] =
        [ins.operands[0], ins.operands[1], ins.operands[2]].map(Vector::decode)
    else {
        return false;
    };
    let Operand::Imm16(index) = ins.operands[3] else {
        return false;
    };
    let width = (dst.bits / 8) as usize;
    let index = index as usize;
    if index >= width {
        return false;
    }
    let regs = emu.regs_aarch64();
    let first = regs.v[left.reg].to_le_bytes();
    let second = regs.v[right.reg].to_le_bytes();
    let mut result = [0; 16];
    let split = width - index;
    result[..split].copy_from_slice(&first[index..width]);
    result[split..width].copy_from_slice(&second[..index]);
    emu.regs_aarch64_mut().v[dst.reg] = u128::from_le_bytes(result);
    true
}

fn zip(emu: &mut Emu, ins: &Instruction) -> bool {
    let [Some(dst), Some(left), Some(right)] =
        [ins.operands[0], ins.operands[1], ins.operands[2]].map(Vector::decode)
    else {
        return false;
    };
    let regs = emu.regs_aarch64();
    let pairs = dst.bits / dst.lane / 2;
    let first = if ins.opcode == Opcode::ZIP2 { pairs } else { 0 };
    let mask = lane_mask(dst.lane);
    let mut result = 0;
    for i in 0..pairs {
        let pos = (first + i) * dst.lane;
        let lo = (regs.v[left.reg] >> pos) & mask;
        let hi = (regs.v[right.reg] >> pos) & mask;
        result |= (lo | (hi << dst.lane)) << (i * dst.lane * 2);
    }
    emu.regs_aarch64_mut().v[dst.reg] = result;
    true
}

fn vector_immediate(emu: &mut Emu, ins: &Instruction) -> bool {
    let Some(dst) = Vector::decode(ins.operands[0]) else {
        return false;
    };
    let lane = match ins.operands[1] {
        Operand::Immediate(value) => value as u128,
        Operand::Imm64(value) => value as u128,
        Operand::ImmShift(value, shift) => (value as u128) << shift,
        Operand::ImmShiftMSL(value, shift) => (value as u128) << shift | lane_mask(shift as u32),
        _ => return false,
    };
    let mut immediate = 0;
    for i in 0..dst.bits / dst.lane {
        immediate |= (lane & lane_mask(dst.lane)) << (i * dst.lane);
    }
    let old = emu.regs_aarch64().v[dst.reg];
    let value = match ins.opcode {
        Opcode::MOVI => immediate,
        Opcode::MVNI => !immediate,
        Opcode::ORR => old | immediate,
        Opcode::BIC => old & !immediate,
        _ => return false,
    };
    emu.regs_aarch64_mut().v[dst.reg] = value & lane_mask(dst.bits);
    true
}

fn exec_dup(emu: &mut Emu, ins: &Instruction) -> bool {
    match (&ins.operands[0], &ins.operands[1]) {
        (Operand::SIMDRegisterElements(_, rd, elem_sz), Operand::Register(_, rn)) => {
            let val = emu.regs_aarch64().get_x(*rn as usize);
            let result = match elem_sz {
                SIMDSizeCode::B => {
                    let b = (val & 0xFF) as u128;
                    b | (b << 8)
                        | (b << 16)
                        | (b << 24)
                        | (b << 32)
                        | (b << 40)
                        | (b << 48)
                        | (b << 56)
                        | (b << 64)
                        | (b << 72)
                        | (b << 80)
                        | (b << 88)
                        | (b << 96)
                        | (b << 104)
                        | (b << 112)
                        | (b << 120)
                }
                SIMDSizeCode::H => {
                    let h = (val & 0xFFFF) as u128;
                    h | (h << 16)
                        | (h << 32)
                        | (h << 48)
                        | (h << 64)
                        | (h << 80)
                        | (h << 96)
                        | (h << 112)
                }
                SIMDSizeCode::S => {
                    let s = (val & 0xFFFFFFFF) as u128;
                    s | (s << 32) | (s << 64) | (s << 96)
                }
                SIMDSizeCode::D => {
                    let d = val as u128;
                    d | (d << 64)
                }
                _ => val as u128,
            };
            emu.regs_aarch64_mut().v[*rd as usize] = result;
        }
        _ => {
            log::warn!("simd: unhandled DUP operand combination");
        }
    }
    true
}

fn exec_fmov(emu: &mut Emu, ins: &Instruction) -> bool {
    match (&ins.operands[0], &ins.operands[1]) {
        (Operand::SIMDRegister(_, rd), Operand::SIMDRegister(_, rn)) => {
            emu.regs_aarch64_mut().v[*rd as usize] = emu.regs_aarch64().v[*rn as usize];
        }
        (Operand::SIMDRegister(sz, rd), Operand::Register(_, rn)) => {
            let val = emu.regs_aarch64().get_x(*rn as usize);
            let v = if matches!(sz, SIMDSizeCode::D) {
                val as u128
            } else {
                (val & 0xffffffff) as u128
            };
            emu.regs_aarch64_mut().v[*rd as usize] = v;
        }
        (Operand::SIMDRegister(..), Operand::ImmediateDouble(v)) => {
            return write_fp(emu, &ins.operands[0], *v);
        }
        (Operand::Register(_, rd), Operand::SIMDRegister(sz, rn)) => {
            let v = emu.regs_aarch64().v[*rn as usize];
            let val = if matches!(sz, SIMDSizeCode::D) {
                v as u64
            } else {
                (v & 0xffffffff) as u64
            };
            emu.regs_aarch64_mut().set_x(*rd as usize, val);
        }
        _ => {}
    }
    true
}

fn elem_bits(sz: SIMDSizeCode) -> u32 {
    match sz {
        SIMDSizeCode::B => 8,
        SIMDSizeCode::H => 16,
        SIMDSizeCode::S => 32,
        SIMDSizeCode::D => 64,
        SIMDSizeCode::Q => 128,
    }
}

fn lane_mask(bits: u32) -> u128 {
    if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    }
}

/// UMOV Rd, Vn.T[i]: zero-extend one vector element into a GPR.
fn exec_umov(emu: &mut Emu, ins: &Instruction) -> bool {
    let (Operand::Register(_, rd), Operand::SIMDRegisterElementsLane(_, rn, sz, lane)) =
        (ins.operands[0], ins.operands[1])
    else {
        return false;
    };
    let bits = elem_bits(sz);
    let v = (emu.regs_aarch64().v[rn as usize] >> (bits * lane as u32)) & lane_mask(bits);
    let size = if bits == 64 { SizeCode::X } else { SizeCode::W };
    write_reg(emu, &Operand::Register(size, rd), v as u64);
    true
}

/// INS Vd.T[i], Rn | Vn.T[j]: replace one vector element.
fn exec_ins(emu: &mut Emu, ins: &Instruction) -> bool {
    let Operand::SIMDRegisterElementsLane(_, rd, sz, lane) = ins.operands[0] else {
        return false;
    };
    let bits = elem_bits(sz);
    let val = match ins.operands[1] {
        Operand::Register(_, rn) => emu.regs_aarch64().get_x(rn as usize) as u128,
        Operand::SIMDRegisterElementsLane(_, rn, _, src_lane) => {
            emu.regs_aarch64().v[rn as usize] >> (bits * src_lane as u32)
        }
        _ => return false,
    };
    let shift = bits * lane as u32;
    let mask = lane_mask(bits) << shift;
    let old = emu.regs_aarch64().v[rd as usize];
    emu.regs_aarch64_mut().v[rd as usize] = (old & !mask) | ((val << shift) & mask);
    true
}

/// NOT/MVN Vd.T, Vn.T: bitwise invert (64-bit forms clear the upper half).
fn exec_not(emu: &mut Emu, ins: &Instruction) -> bool {
    let (Operand::SIMDRegisterElements(q, rd, _), Operand::SIMDRegisterElements(_, rn, _)) =
        (ins.operands[0], ins.operands[1])
    else {
        return false;
    };
    let width = if matches!(q, SIMDSizeCode::Q) {
        128
    } else {
        64
    };
    emu.regs_aarch64_mut().v[rd as usize] = !emu.regs_aarch64().v[rn as usize] & lane_mask(width);
    true
}
