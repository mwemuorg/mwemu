use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Operand, SIMDSizeCode, ShiftStyle, SizeCode};

pub enum ShiftOp {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

/// Strip ARMv8.3 Pointer Authentication Code bits from an address.
/// Userspace macOS uses 47-bit virtual addresses; bits [62:47] carry
/// the PAC signature and bit 63 selects kernel/user space.
pub fn pac_strip(addr: u64) -> u64 {
    if addr & (1 << 55) != 0 {
        addr | 0xFFFF_0000_0000_0000
    } else {
        addr & 0x0000_FFFF_FFFF_FFFF
    }
}

pub fn is_64(sz: &SizeCode) -> bool {
    matches!(sz, SizeCode::X)
}

pub fn operand_is_64(op: &Operand) -> bool {
    match op {
        Operand::Register(sz, _) | Operand::RegisterOrSP(sz, _) => is_64(sz),
        _ => true,
    }
}

pub fn read_reg(emu: &Emu, op: &Operand) -> u64 {
    let regs = emu.regs_aarch64();
    match op {
        Operand::Register(sz, n) => {
            let val = regs.get_x(*n as usize);
            if is_64(sz) { val } else { val & 0xffffffff }
        }
        Operand::RegisterOrSP(sz, n) => {
            let val = regs.get_x_or_sp(*n as usize);
            if is_64(sz) { val } else { val & 0xffffffff }
        }
        _ => unreachable!("expected register operand, got {:?}", op),
    }
}

pub fn write_reg(emu: &mut Emu, op: &Operand, val: u64) {
    match op {
        Operand::Register(sz, n) => {
            let val = if is_64(sz) { val } else { val & 0xffffffff };
            emu.regs_aarch64_mut().set_x(*n as usize, val);
        }
        Operand::RegisterOrSP(sz, n) => {
            let val = if is_64(sz) { val } else { val & 0xffffffff };
            emu.regs_aarch64_mut().set_x_or_sp(*n as usize, val);
        }
        _ => unreachable!("expected register operand, got {:?}", op),
    }
}

pub fn read_imm(op: &Operand) -> u64 {
    match op {
        Operand::Immediate(v) => *v as u64,
        Operand::Imm64(v) => *v,
        Operand::Imm16(v) => *v as u64,
        _ => unreachable!("expected immediate operand, got {:?}", op),
    }
}

pub fn read_operand_value(emu: &Emu, op: &Operand) -> u64 {
    match op {
        Operand::Register(..) | Operand::RegisterOrSP(..) => read_reg(emu, op),
        Operand::Immediate(v) => *v as u64,
        Operand::Imm64(v) => *v,
        Operand::Imm16(v) => *v as u64,
        Operand::ImmShift(v, shift) => (*v as u64) << (*shift as u64),
        Operand::RegShift(style, amt, sz, reg) => {
            let val = emu.regs_aarch64().get_x(*reg as usize);
            if is_64(sz) {
                apply_shift(val, *style, *amt as u32)
            } else {
                apply_shift32(val as u32, *style, *amt as u32)
            }
        }
        Operand::SIMDRegister(_, reg) | Operand::SIMDRegisterElements(_, reg, _) => {
            emu.regs_aarch64().v[*reg as usize] as u64
        }
        _ => unreachable!("unsupported operand for read_operand_value: {:?}", op),
    }
}

pub fn apply_shift(val: u64, style: ShiftStyle, amt: u32) -> u64 {
    match style {
        ShiftStyle::LSL => val << amt,
        ShiftStyle::LSR => val >> amt,
        ShiftStyle::ASR => ((val as i64) >> amt) as u64,
        ShiftStyle::ROR => val.rotate_right(amt),
        ShiftStyle::UXTB => (val as u8) as u64,
        ShiftStyle::UXTH => (val as u16) as u64,
        ShiftStyle::UXTW => (val as u32) as u64,
        ShiftStyle::UXTX => val,
        ShiftStyle::SXTB => (val as i8) as i64 as u64,
        ShiftStyle::SXTH => (val as i16) as i64 as u64,
        ShiftStyle::SXTW => (val as i32) as i64 as u64,
        ShiftStyle::SXTX => val,
    }
}

/// Shift a 32-bit (W) register operand; ASR and ROR must act on bit 31.
pub fn apply_shift32(val: u32, style: ShiftStyle, amt: u32) -> u64 {
    match style {
        ShiftStyle::LSL => val.wrapping_shl(amt) as u64,
        ShiftStyle::LSR => val.wrapping_shr(amt) as u64,
        ShiftStyle::ASR => ((val as i32).wrapping_shr(amt) as u32) as u64,
        ShiftStyle::ROR => val.rotate_right(amt) as u64,
        _ => apply_shift(val as u64, style, amt),
    }
}

/// Read `bytes` (1, 2, 4 or 8) little-endian bytes, zero-extended.
pub fn read_mem(emu: &Emu, addr: u64, bytes: u64) -> Option<u64> {
    match bytes {
        1 => emu.maps.read_byte(addr).map(u64::from),
        2 => emu.maps.read_word(addr).map(u64::from),
        4 => emu.maps.read_dword(addr).map(u64::from),
        _ => emu.maps.read_qword(addr),
    }
}

/// Write the low `bytes` (1, 2, 4 or 8) bytes of `val`.
pub fn write_mem(emu: &mut Emu, addr: u64, bytes: u64, val: u64) -> bool {
    match bytes {
        1 => emu.maps.write_byte(addr, val as u8),
        2 => emu.maps.write_word(addr, val as u16),
        4 => emu.maps.write_dword(addr, val as u32),
        _ => emu.maps.write_qword(addr, val),
    }
}

/// Access width in bytes of a W/X register operand.
pub fn reg_bytes(op: &Operand) -> u64 {
    if operand_is_64(op) { 8 } else { 4 }
}

/// Resolve a memory operand, returning (effective_address, writeback_info).
pub fn resolve_mem_addr(emu: &Emu, op: &Operand) -> (u64, Option<(usize, u64)>) {
    let regs = emu.regs_aarch64();
    match op {
        Operand::RegPreIndex(reg, offset, writeback) => {
            let base = regs.get_x_or_sp(*reg as usize);
            let addr = base.wrapping_add(*offset as i64 as u64);
            let wb = if *writeback {
                Some((*reg as usize, addr))
            } else {
                None
            };
            (addr, wb)
        }
        Operand::RegPostIndex(reg, offset) => {
            let base = regs.get_x_or_sp(*reg as usize);
            let new_base = base.wrapping_add(*offset as i64 as u64);
            (base, Some((*reg as usize, new_base)))
        }
        Operand::RegRegOffset(base_reg, index_reg, index_size, shift_style, shift_amt) => {
            let base = regs.get_x_or_sp(*base_reg as usize);
            let mut index = regs.get_x(*index_reg as usize);
            if !is_64(index_size) {
                index &= 0xffffffff;
            }
            index = apply_shift(index, *shift_style, *shift_amt as u32);
            (base.wrapping_add(index), None)
        }
        Operand::PCOffset(offset) => {
            let addr = regs.pc.wrapping_add(*offset as u64);
            (addr, None)
        }
        _ => unreachable!("unsupported memory operand: {:?}", op),
    }
}

pub fn do_writeback(emu: &mut Emu, wb: Option<(usize, u64)>) {
    if let Some((reg, val)) = wb {
        emu.regs_aarch64_mut().set_x_or_sp(reg, val);
    }
}

/// Read a scalar S or D floating-point register as f64.
pub fn read_fp(emu: &Emu, op: &Operand) -> Option<f64> {
    let Operand::SIMDRegister(sz, r) = op else {
        return None;
    };
    let raw = emu.regs_aarch64().v[*r as usize];
    match sz {
        SIMDSizeCode::S => Some(f32::from_bits(raw as u32) as f64),
        SIMDSizeCode::D => Some(f64::from_bits(raw as u64)),
        _ => None,
    }
}

/// Write a scalar S or D floating-point register, zeroing the upper lanes.
pub fn write_fp(emu: &mut Emu, op: &Operand, val: f64) -> bool {
    let Operand::SIMDRegister(sz, r) = op else {
        return false;
    };
    let raw = match sz {
        SIMDSizeCode::S => (val as f32).to_bits() as u128,
        SIMDSizeCode::D => val.to_bits() as u128,
        _ => return false,
    };
    emu.regs_aarch64_mut().v[*r as usize] = raw;
    true
}
