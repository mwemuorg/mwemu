use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Operand, ShiftStyle, SizeCode};

pub enum ShiftOp {
    Lsl,
    Lsr,
    Asr,
    Ror,
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
        _ => panic!("expected register operand, got {:?}", op),
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
        _ => panic!("expected register operand, got {:?}", op),
    }
}

pub fn read_imm(op: &Operand) -> u64 {
    match op {
        Operand::Immediate(v) => *v as u64,
        Operand::Imm64(v) => *v,
        Operand::Imm16(v) => *v as u64,
        _ => panic!("expected immediate operand, got {:?}", op),
    }
}

pub fn read_operand_value(emu: &Emu, op: &Operand) -> u64 {
    match op {
        Operand::Register(..) | Operand::RegisterOrSP(..) => read_reg(emu, op),
        Operand::Immediate(v) => *v as u64,
        Operand::Imm64(v) => *v,
        Operand::Imm16(v) => *v as u64,
        Operand::ImmShift(v, shift) => (*v as u64) << (*shift as u64),
        Operand::RegShift(style, amt, _sz, reg) => {
            let val = emu.regs_aarch64().get_x(*reg as usize);
            apply_shift(val, *style, *amt as u32)
        }
        _ => panic!("unsupported operand for read_operand_value: {:?}", op),
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
        _ => panic!("unsupported memory operand: {:?}", op),
    }
}

pub fn do_writeback(emu: &mut Emu, wb: Option<(usize, u64)>) {
    if let Some((reg, val)) = wb {
        emu.regs_aarch64_mut().set_x_or_sp(reg, val);
    }
}
