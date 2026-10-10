use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// ADC/ADCS/SBC/SBCS (and NGC/NGCS). Subtraction is Rn + !Rm + C.
pub fn execute(emu: &mut Emu, ins: &Instruction, subtract: bool, set_flags: bool) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_reg(emu, &ins.operands[1]);
    let b = read_reg(emu, &ins.operands[2]);
    let b = if subtract { !b } else { b };
    let carry_in = emu.regs_aarch64().nzcv.c as u64;
    let bits = if is64 { 64 } else { 32 };
    let mask = if is64 { u64::MAX } else { 0xffff_ffff };
    let (a, b) = (a & mask, b & mask);

    let wide = a as u128 + b as u128 + carry_in as u128;
    let result = (wide as u64) & mask;

    if set_flags {
        let sign = 1u64 << (bits - 1);
        let nzcv = &mut emu.regs_aarch64_mut().nzcv;
        nzcv.n = result & sign != 0;
        nzcv.z = result == 0;
        nzcv.c = wide >> bits != 0;
        nzcv.v = (a & sign) == (b & sign) && (result & sign) != (a & sign);
    }
    write_reg(emu, &ins.operands[0], result);
    true
}
