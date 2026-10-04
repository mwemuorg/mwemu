use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// BICS Rd, Rn, Rm{, shift}: Rd = Rn & !Rm, setting NZ and clearing CV.
pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let result = a & !b;
    if is64 {
        emu.regs_aarch64_mut().nzcv.update_logic64(result);
    } else {
        emu.regs_aarch64_mut().nzcv.update_logic32(result as u32);
    }
    write_reg(emu, &ins.operands[0], result);
    true
}
