use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // MRS Xt, <sysreg> — read system register into Xt
    // For now, return 0 for most system registers
    let val = 0u64; // TODO: implement specific system registers
    write_reg(emu, &ins.operands[0], val);
    true
}
