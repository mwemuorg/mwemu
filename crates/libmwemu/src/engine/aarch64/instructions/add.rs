use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction, set_flags: bool) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let result = a.wrapping_add(b);

    if set_flags {
        if is64 {
            emu.regs_aarch64_mut().nzcv.update_add64(a, b, result);
        } else {
            emu.regs_aarch64_mut()
                .nzcv
                .update_add32(a as u32, b as u32, result as u32);
        }
    }

    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
