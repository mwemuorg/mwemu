use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// BFM Rd, Rn, #immr, #imms (also BFI/BFXIL): copy a bitfield of Rn into Rd,
/// leaving the other bits of Rd untouched.
pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let bits = if operand_is_64(&ins.operands[0]) {
        64
    } else {
        32
    };
    let dst = read_reg(emu, &ins.operands[0]);
    let src = read_reg(emu, &ins.operands[1]);
    let immr = read_imm(&ins.operands[2]) as u32;
    let imms = read_imm(&ins.operands[3]) as u32;

    let (field, lsb, width) = if imms >= immr {
        // BFXIL: Rn<imms:immr> -> Rd<width-1:0>
        let width = imms - immr + 1;
        (src >> immr, 0, width)
    } else {
        // BFI: Rn<imms:0> -> Rd<bits-immr+imms : bits-immr>
        (src, bits - immr, imms + 1)
    };
    let mask = low_mask(width) << lsb;
    let result = (dst & !mask) | ((field << lsb) & mask);
    write_reg(emu, &ins.operands[0], result);
    true
}

fn low_mask(width: u32) -> u64 {
    if width >= 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    }
}
