use crate::emu;

pub fn GetStringTypeW_api(emu: &mut emu::Emu) {
    let dw_info_type = emu.regs().rcx as u32;
    let lp_src_str = emu.regs().rdx;
    let cch_src = emu.regs().r8 as i32;
    let lp_char_type = emu.regs().r9;

    let actual_len = if cch_src == -1 {
        let s = emu.maps.read_wide_string(lp_src_str);
        s.len() + 1
    } else {
        cch_src as usize
    };

    if lp_char_type != 0 && actual_len > 0 {
        for i in 0..actual_len {
            let ch = emu.maps.read_word(lp_src_str + (i as u64) * 2).unwrap_or(0);
            let ct = match dw_info_type {
                1 => {
                    // CT_CTYPE1
                    let mut t: u16 = 0;
                    if (ch as u8 as char).is_ascii_uppercase() {
                        t |= 0x0001; // C1_UPPER
                    }
                    if (ch as u8 as char).is_ascii_lowercase() {
                        t |= 0x0002; // C1_LOWER
                    }
                    if (ch as u8 as char).is_ascii_digit() {
                        t |= 0x0004; // C1_DIGIT
                    }
                    if ch == b' ' as u16 || ch == b'\t' as u16 {
                        t |= 0x0008; // C1_SPACE
                    }
                    if (ch as u8 as char).is_ascii_punctuation() {
                        t |= 0x0010; // C1_PUNCT
                    }
                    if ch < 0x20 || ch == 0x7f {
                        t |= 0x0020; // C1_CNTRL
                    }
                    if (ch as u8 as char).is_ascii_alphanumeric() {
                        t |= 0x0100; // C1_ALPHA
                    }
                    t
                }
                2 if (ch as u8 as char).is_ascii_alphanumeric() => {
                    2 // CT_CTYPE2 — C2_LEFTTORIGHT
                }
                2 => {
                    0 // CT_CTYPE2 — C2_NOTAPPLICABLE
                }
                _ => 0,
            };
            emu.maps.write_word(lp_char_type + (i as u64) * 2, ct);
        }
    }

    log_red!(
        emu,
        "kernel32!GetStringTypeW type:{} len:{} => TRUE",
        dw_info_type,
        actual_len
    );
    emu.regs_mut().rax = 1;
}
