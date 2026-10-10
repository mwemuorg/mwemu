use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// SMADDL/UMADDL/SMSUBL/UMSUBL (and SMULL/UMULL/SMNEGL/UMNEGL aliases):
/// Xd = Xa +/- (Wn * Wm) with 32-bit sources widened to 64 bits.
pub fn execute(emu: &mut Emu, ins: &Instruction, signed: bool, subtract: bool) -> bool {
    let n = read_reg(emu, &ins.operands[1]);
    let m = read_reg(emu, &ins.operands[2]);
    let a = read_reg(emu, &ins.operands[3]);
    let product = if signed {
        (n as u32 as i32 as i64).wrapping_mul(m as u32 as i32 as i64) as u64
    } else {
        (n as u32 as u64).wrapping_mul(m as u32 as u64)
    };
    let result = if subtract {
        a.wrapping_sub(product)
    } else {
        a.wrapping_add(product)
    };
    write_reg(emu, &ins.operands[0], result);
    true
}
