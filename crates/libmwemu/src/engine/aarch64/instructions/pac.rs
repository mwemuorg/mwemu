use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// AUTIA/AUTIB/AUTDA/AUTDB Xd, Xn — strip PAC from Xd (operand 0).
/// AUTIZA/AUTIZB/AUTDZA/AUTDZB Xd — strip PAC from Xd (operand 0).
/// XPACI/XPACD Xd — strip PAC from Xd (operand 0).
pub fn execute_aut_reg(emu: &mut Emu, ins: &Instruction) -> bool {
    let val = read_reg(emu, &ins.operands[0]);
    let stripped = pac_strip(val);
    write_reg(emu, &ins.operands[0], stripped);
    true
}

/// AUTIASP/AUTIBSP/AUTIAZ/AUTIBZ etc. — strip PAC from x30 (LR).
pub fn execute_aut_implicit(emu: &mut Emu, _ins: &Instruction) -> bool {
    let lr = emu.regs_aarch64().x[30];
    emu.regs_aarch64_mut().x[30] = pac_strip(lr);
    true
}
