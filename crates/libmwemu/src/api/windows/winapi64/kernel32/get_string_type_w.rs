use crate::emu;

pub fn GetStringTypeW(emu: &mut emu::Emu) {
    let info_type = emu.regs().rcx as u32;
    let str_ptr = emu.regs().rdx;
    let str_len = emu.regs().r8 as i32;
    let char_type_ptr = emu.regs().r9;

    let ustr = emu.maps.read_wide_string(str_ptr);

    log_red!(
        emu,
        "kernel32!GetStringTypeW type:{} '{}' len:{}",
        info_type,
        ustr,
        str_len
    );

    let len = if str_len < 0 {
        ustr.chars().count()
    } else {
        str_len as usize
    };

    if char_type_ptr != 0 {
        for i in 0..len {
            let ch = ustr.chars().nth(i).unwrap_or('\0');
            let ct: u16 = match info_type {
                1 => {
                    // CT_CTYPE1
                    let mut flags = 0u16;
                    if ch.is_uppercase() {
                        flags |= 0x0001; // C1_UPPER
                    }
                    if ch.is_lowercase() {
                        flags |= 0x0002; // C1_LOWER
                    }
                    if ch.is_ascii_digit() {
                        flags |= 0x0004; // C1_DIGIT
                    }
                    if ch == ' ' || ch == '\t' {
                        flags |= 0x0008; // C1_SPACE
                    }
                    if ch.is_ascii_punctuation() {
                        flags |= 0x0010; // C1_PUNCT
                    }
                    if ch.is_ascii_control() {
                        flags |= 0x0020; // C1_CNTRL
                    }
                    if ch.is_alphanumeric() {
                        flags |= 0x0100; // C1_ALPHA
                    }
                    if flags == 0 {
                        flags = 0x0100; // default: alpha
                    }
                    flags
                }
                _ => 0,
            };
            emu.maps
                .write_word(char_type_ptr + (i as u64) * 2, ct);
        }
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
