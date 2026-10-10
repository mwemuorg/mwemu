use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let src = read_operand_value(emu, &ins.operands[1]);
    let immr = read_operand_value(emu, &ins.operands[2]) as u32;
    let imms = read_operand_value(emu, &ins.operands[3]) as u32;
    let bits = if is64 { 64u32 } else { 32 };

    let result = if imms >= immr {
        let width = imms - immr + 1;
        let field = (src >> immr) & ((1u64 << width) - 1);
        sign_extend(field, width, bits)
    } else {
        let width = imms + 1;
        let field = src & ((1u64 << width) - 1);
        let pos = bits - immr;
        let placed = field << pos;
        sign_extend_from(placed, pos + width - 1, bits)
    };

    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}

fn sign_extend(val: u64, width: u32, total: u32) -> u64 {
    if width == 0 || width >= total {
        return val;
    }
    let sign_bit = (val >> (width - 1)) & 1;
    if sign_bit == 1 {
        let mask = !((1u64 << width) - 1);
        let mask = if total < 64 {
            mask & ((1u64 << total) - 1)
        } else {
            mask
        };
        val | mask
    } else {
        val
    }
}

fn sign_extend_from(val: u64, msb: u32, total: u32) -> u64 {
    if msb + 1 >= total {
        return val;
    }
    let sign_bit = (val >> msb) & 1;
    if sign_bit == 1 {
        let mask = !((1u64 << (msb + 1)) - 1);
        let mask = if total < 64 {
            mask & ((1u64 << total) - 1)
        } else {
            mask
        };
        val | mask
    } else {
        val & ((1u64 << (msb + 1)) - 1)
    }
}
