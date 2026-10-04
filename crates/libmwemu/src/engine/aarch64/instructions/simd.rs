use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode, Operand, SIMDSizeCode, SizeCode};

use super::super::helpers::{write_fp, write_reg};

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    match ins.opcode {
        Opcode::MOVI => exec_movi(emu, ins),
        Opcode::FMOV => exec_fmov(emu, ins),
        Opcode::DUP => exec_dup(emu, ins),
        Opcode::UMOV => exec_umov(emu, ins),
        Opcode::INS => exec_ins(emu, ins),
        Opcode::NOT => exec_not(emu, ins),
        _ => {
            log::warn!("simd: unhandled opcode {:?}", ins.opcode);
            true
        }
    }
}

fn exec_movi(emu: &mut Emu, ins: &Instruction) -> bool {
    match ins.operands[0] {
        Operand::SIMDRegisterElements(_, reg, _) | Operand::SIMDRegister(_, reg) => {
            let imm = match ins.operands[1] {
                Operand::Immediate(v) => v as u128,
                Operand::ImmShift(v, shift) => (v as u128) << shift,
                _ => 0,
            };
            emu.regs_aarch64_mut().v[reg as usize] = imm;
            true
        }
        _ => true,
    }
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
