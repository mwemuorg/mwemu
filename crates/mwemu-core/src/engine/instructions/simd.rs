//! Shared operand and lane operations for MMX/SSE instruction handlers.

use iced_x86::{Instruction, OpKind};

use crate::emu::Emu;
use crate::utils::bits::{low_bits_u64, low_bits_u128};

pub fn binop(emu: &mut Emu, ins: &Instruction, operation: impl FnOnce(u128, u128) -> u128) -> bool {
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    emu.set_operand_xmm_value_128(ins, 0, operation(dest, src))
}

pub fn unop(emu: &mut Emu, ins: &Instruction, operation: impl FnOnce(u128) -> u128) -> bool {
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    emu.set_operand_xmm_value_128(ins, 0, operation(src))
}

pub fn scalar32(emu: &mut Emu, ins: &Instruction, operation: impl FnOnce(f32, f32) -> f32) -> bool {
    binop(emu, ins, |dest, src| {
        let result = operation(f32::from_bits(dest as u32), f32::from_bits(src as u32));
        (dest & !low_bits_u128(u128::MAX, u32::BITS)) | result.to_bits() as u128
    })
}

pub fn scalar64(emu: &mut Emu, ins: &Instruction, operation: impl FnOnce(f64, f64) -> f64) -> bool {
    binop(emu, ins, |dest, src| {
        let result = operation(f64::from_bits(dest as u64), f64::from_bits(src as u64));
        (dest & !low_bits_u128(u128::MAX, u64::BITS)) | result.to_bits() as u128
    })
}

pub enum Shift {
    Left,
    Right,
    Arithmetic,
}

/// MMX and SSE packed shifts use the entire low qword of a register count.
pub fn shift(emu: &mut Emu, ins: &Instruction, bits: u32, kind: Shift) -> bool {
    let Some(value) = emu.get_operand_xmm_value_128(ins, 0, true) else {
        return false;
    };
    let Some(count) = emu.get_operand_xmm_value_128(ins, 1, true) else {
        return false;
    };
    let count = count as u64;
    let mut result = 0;
    for offset in (0..emu.get_operand_sz(ins, 0)).step_by(bits as usize) {
        let lane = low_bits_u128(value >> offset, bits);
        let shifted = match kind {
            Shift::Arithmetic => {
                let signed = ((lane << (u128::BITS - bits)) as i128) >> (u128::BITS - bits);
                low_bits_u128((signed >> count.min((bits - 1) as u64)) as u128, bits)
            }
            _ if count >= bits as u64 => 0,
            Shift::Left => low_bits_u128(lane << count, bits),
            Shift::Right => lane >> count,
        };
        result |= shifted << offset;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}

/// Pack both operands into the destination width, saturating each signed input lane.
pub fn pack(emu: &mut Emu, ins: &Instruction, bits: u32, signed: bool) -> bool {
    let Some(a) = emu.get_operand_xmm_value_128(ins, 0, true) else {
        return false;
    };
    let Some(b) = emu.get_operand_xmm_value_128(ins, 1, true) else {
        return false;
    };
    let output_bits = bits / 2;
    let mask = low_bits_u128(u128::MAX, output_bits);
    let (min, max) = if signed {
        let limit = 1i128 << (output_bits - 1);
        (-limit, limit - 1)
    } else {
        (0, mask as i128)
    };
    let lanes = emu.get_operand_sz(ins, 0) / bits;
    let mut result = 0;
    for (operand, value) in [a, b].into_iter().enumerate() {
        for lane in 0..lanes {
            let value = value >> (lane * bits);
            let signed = ((value << (u128::BITS - bits)) as i128) >> (u128::BITS - bits);
            let packed = low_bits_u128(signed.clamp(min, max) as u128, output_bits);
            result |= packed << ((operand as u32 * lanes + lane) * output_bits);
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}

/// MOVD/MOVQ share scalar transfers to and from MMX/XMM registers.
pub fn move_scalar(emu: &mut Emu, ins: &Instruction, bits: u32) -> bool {
    let source_is_simd =
        ins.op1_kind() == OpKind::Register && emu.regs().is_xmm(ins.op1_register());
    let value = if source_is_simd {
        emu.get_operand_xmm_value_128(ins, 1, true)
            .map(|value| value as u64)
    } else {
        emu.get_operand_value(ins, 1, true)
    };
    let Some(value) = value else {
        return false;
    };
    let value = low_bits_u64(value, bits);
    if ins.op0_kind() == OpKind::Register && emu.regs().is_xmm(ins.op0_register()) {
        emu.set_operand_xmm_value_128(ins, 0, value as u128)
    } else {
        emu.set_operand_value(ins, 0, value)
    }
}

/// Interleave the low or high half of each operand, with destination lanes first.
pub fn unpack(emu: &mut Emu, ins: &Instruction, bits: u32, high: bool) -> bool {
    let Some(a) = emu.get_operand_xmm_value_128(ins, 0, true) else {
        return false;
    };
    let Some(b) = emu.get_operand_xmm_value_128(ins, 1, true) else {
        return false;
    };
    let half = emu.get_operand_sz(ins, 0) / 2;
    let source_offset = if high { half } else { 0 };
    let mut result = 0;
    for lane in 0..half / bits {
        let shift = source_offset + lane * bits;
        result |= low_bits_u128(a >> shift, bits) << (lane * 2 * bits);
        result |= low_bits_u128(b >> shift, bits) << ((lane * 2 + 1) * bits);
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}

/// Apply a lane operation at the decoded MMX/SSE width and discard lane overflow.
pub fn binary_lanes(
    emu: &mut Emu,
    ins: &Instruction,
    bits: u32,
    operation: impl Fn(u128, u128) -> u128,
) -> bool {
    let Some(a) = emu.get_operand_xmm_value_128(ins, 0, true) else {
        return false;
    };
    let Some(b) = emu.get_operand_xmm_value_128(ins, 1, true) else {
        return false;
    };
    let mut result = 0;
    for offset in (0..emu.get_operand_sz(ins, 0)).step_by(bits as usize) {
        let lane = operation(
            low_bits_u128(a >> offset, bits),
            low_bits_u128(b >> offset, bits),
        );
        result |= low_bits_u128(lane, bits) << offset;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}

/// Combine adjacent lanes from each operand into its half of the destination.
pub fn horizontal(
    emu: &mut Emu,
    ins: &Instruction,
    bits: u32,
    operation: impl Fn(u128, u128) -> u128,
) -> bool {
    let Some(a) = emu.get_operand_xmm_value_128(ins, 0, true) else {
        return false;
    };
    let Some(b) = emu.get_operand_xmm_value_128(ins, 1, true) else {
        return false;
    };
    let pairs = emu.get_operand_sz(ins, 0) / (2 * bits);
    let mut result = 0;
    for (operand, value) in [a, b].into_iter().enumerate() {
        for pair in 0..pairs {
            let first = low_bits_u128(value >> (pair * 2 * bits), bits);
            let second = low_bits_u128(value >> ((pair * 2 + 1) * bits), bits);
            let lane = low_bits_u128(operation(first, second), bits);
            result |= lane << ((operand as u32 * pairs + pair) * bits);
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}

/// Byte shifts saturate at the operand width instead of wrapping the count.
pub fn shift_bytes_left(value: u128, count: u64) -> u128 {
    let bits = count.min(size_of::<u128>() as u64) as u32 * u8::BITS;
    value.checked_shl(bits).unwrap_or(0)
}

pub fn shift_bytes_right(value: u128, count: u64) -> u128 {
    let bits = count.min(size_of::<u128>() as u64) as u32 * u8::BITS;
    value.checked_shr(bits).unwrap_or(0)
}
