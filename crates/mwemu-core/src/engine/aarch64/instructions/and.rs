use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction, set_flags: bool) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let result = a & b;

    if set_flags {
        if is64 {
            emu.regs_aarch64_mut().nzcv.update_logic64(result);
        } else {
            emu.regs_aarch64_mut().nzcv.update_logic32(result as u32);
        }
    }

    // ANDS with XZR dest = TST (flag-only)
    if set_flags && matches!(ins.operands[0], Operand::Register(_, 31)) {
        return true;
    }

    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
