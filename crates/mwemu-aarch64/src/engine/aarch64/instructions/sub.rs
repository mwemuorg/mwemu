use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand, SIMDSizeCode};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction, set_flags: bool) -> bool {
    if matches!(
        ins.operands[0],
        Operand::SIMDRegisterElements(..) | Operand::SIMDRegister(..)
    ) {
        return execute_simd(emu, ins);
    }

    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let result = a.wrapping_sub(b);

    if set_flags {
        if is64 {
            emu.regs_aarch64_mut().nzcv.update_sub64(a, b, result);
        } else {
            emu.regs_aarch64_mut()
                .nzcv
                .update_sub32(a as u32, b as u32, result as u32);
        }
    }

    if set_flags && matches!(ins.operands[0], Operand::Register(_, 31)) {
        return true;
    }

    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}

fn execute_simd(emu: &mut Emu, ins: &Instruction) -> bool {
    let (rd, elem_sz) = match ins.operands[0] {
        Operand::SIMDRegisterElements(_, r, sz) => (r, sz),
        Operand::SIMDRegister(sz, r) => (r, sz),
        _ => return true,
    };
    let a = read_simd_reg(emu, &ins.operands[1]);
    let b = read_simd_reg(emu, &ins.operands[2]);

    let elem_bits = match elem_sz {
        SIMDSizeCode::B => 8,
        SIMDSizeCode::H => 16,
        SIMDSizeCode::S => 32,
        SIMDSizeCode::D => 64,
        _ => 64,
    };
    let result = simd_sub(a, b, elem_bits);
    emu.regs_aarch64_mut().v[rd as usize] = result;
    true
}

fn read_simd_reg(emu: &Emu, op: &Operand) -> u128 {
    match op {
        Operand::SIMDRegisterElements(_, r, _) | Operand::SIMDRegister(_, r) => {
            emu.regs_aarch64().v[*r as usize]
        }
        _ => 0,
    }
}

fn simd_sub(a: u128, b: u128, elem_bits: u32) -> u128 {
    let mask: u128 = if elem_bits >= 128 {
        u128::MAX
    } else {
        (1u128 << elem_bits) - 1
    };
    let count = 128 / elem_bits;
    let mut result = 0u128;
    for i in 0..count {
        let shift = i * elem_bits;
        let va = (a >> shift) & mask;
        let vb = (b >> shift) & mask;
        let vr = va.wrapping_sub(vb) & mask;
        result |= vr << shift;
    }
    result
}
