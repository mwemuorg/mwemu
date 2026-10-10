use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::{operand_is_64, read_operand_value, read_reg};

/// CCMP: if condition holds, compare Rn-Rm and set flags; else set NZCV to imm.
/// CCMN: if condition holds, compare Rn+Rm and set flags; else set NZCV to imm.
pub fn execute(emu: &mut Emu, ins: &Instruction, is_sub: bool) -> bool {
    let cond = match ins.operands[3] {
        Operand::ConditionCode(c) => c,
        _ => return false,
    };
    let nzcv_imm = match ins.operands[2] {
        Operand::Immediate(v) => v as u8,
        _ => return false,
    };

    if emu.regs_aarch64().nzcv.eval_condition(cond) {
        let is64 = operand_is_64(&ins.operands[0]);
        let a = read_reg(emu, &ins.operands[0]);
        let b = read_operand_value(emu, &ins.operands[1]);
        if is_sub {
            let result = a.wrapping_sub(b);
            if is64 {
                emu.regs_aarch64_mut().nzcv.update_sub64(a, b, result);
            } else {
                emu.regs_aarch64_mut()
                    .nzcv
                    .update_sub32(a as u32, b as u32, result as u32);
            }
        } else {
            let result = a.wrapping_add(b);
            if is64 {
                emu.regs_aarch64_mut().nzcv.update_add64(a, b, result);
            } else {
                emu.regs_aarch64_mut()
                    .nzcv
                    .update_add32(a as u32, b as u32, result as u32);
            }
        }
    } else {
        let nzcv = &mut emu.regs_aarch64_mut().nzcv;
        nzcv.n = nzcv_imm & 0x8 != 0;
        nzcv.z = nzcv_imm & 0x4 != 0;
        nzcv.c = nzcv_imm & 0x2 != 0;
        nzcv.v = nzcv_imm & 0x1 != 0;
    }
    true
}
