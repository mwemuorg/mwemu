use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PCMPISTRM xmm1, xmm2/m128, imm8 : SSE4.2 packed compare of implicit-length
// strings, returning a mask in XMM0 and setting flags.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let op0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let op1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let imm = ins.immediate8();

    let is_word = imm & 1 != 0;
    let is_signed = imm & 2 != 0;
    let agg = (imm >> 2) & 3;
    let polarity = (imm >> 4) & 3;
    let unit_mask = (imm >> 6) & 1 != 0; // 0 = bit mask, 1 = byte/word mask

    let n = if is_word { 8 } else { 16 };
    let mask: u32 = if is_word { 0xff } else { 0xffff };

    let elem = |val: u128, idx: usize| -> i32 {
        if is_word {
            let w = ((val >> (idx * 16)) & 0xffff) as u16;
            if is_signed {
                (w as i16) as i32
            } else {
                w as i32
            }
        } else {
            let b = ((val >> (idx * 8)) & 0xff) as u8;
            if is_signed {
                (b as i8) as i32
            } else {
                b as i32
            }
        }
    };
    let is_null = |val: u128, idx: usize| -> bool {
        if is_word {
            ((val >> (idx * 16)) & 0xffff) == 0
        } else {
            ((val >> (idx * 8)) & 0xff) == 0
        }
    };

    let mut len1 = n;
    let mut len2 = n;
    for i in 0..n {
        if is_null(op0, i) {
            len1 = i;
            break;
        }
    }
    for i in 0..n {
        if is_null(op1, i) {
            len2 = i;
            break;
        }
    }
    let valid1 = |j: usize| j < len1;
    let valid2 = |i: usize| i < len2;

    let overridden = |j: usize, i: usize| -> bool {
        match (valid1(j), valid2(i)) {
            (true, true) => elem(op0, j) == elem(op1, i),
            (false, false) => agg == 2 || agg == 3,
            (false, true) => agg == 3,
            (true, false) => false,
        }
    };

    let mut intres1: u32 = 0;
    for i in 0..n {
        let bit = match agg {
            0 => (0..n).any(|j| overridden(j, i)),
            1 => {
                let mut r = false;
                let mut j = 0;
                while j + 1 < n {
                    if valid2(i) && valid1(j) && valid1(j + 1) {
                        let x = elem(op1, i);
                        if x >= elem(op0, j) && x <= elem(op0, j + 1) {
                            r = true;
                        }
                    }
                    j += 2;
                }
                r
            }
            2 => overridden(i, i),
            _ => (0..(n - i)).all(|j| overridden(j, i + j)),
        };
        if bit {
            intres1 |= 1 << i;
        }
    }

    let intres2 = match polarity {
        1 => (!intres1) & mask,
        3 => {
            let mut r = intres1;
            for i in 0..n {
                if valid2(i) {
                    r ^= 1 << i;
                }
            }
            r & mask
        }
        _ => intres1 & mask,
    };

    // Build the XMM0 result.
    let result: u128 = if unit_mask {
        // expand each set bit into a full element (0xFF byte / 0xFFFF word)
        let mut r: u128 = 0;
        for i in 0..n {
            if intres2 & (1 << i) != 0 {
                if is_word {
                    r |= 0xffffu128 << (i * 16);
                } else {
                    r |= 0xffu128 << (i * 8);
                }
            }
        }
        r
    } else {
        // zero-extended bit mask in the low bits
        intres2 as u128
    };

    // PCMPISTRM always targets XMM0.
    emu.regs_mut().set_xmm_reg(iced_x86::Register::XMM0, result);

    emu.flags_mut().f_cf = intres2 != 0;
    emu.flags_mut().f_zf = len2 < n;
    emu.flags_mut().f_sf = len1 < n;
    emu.flags_mut().f_of = (intres2 & 1) != 0;
    emu.flags_mut().f_af = false;
    emu.flags_mut().f_pf = false;
    true
}
