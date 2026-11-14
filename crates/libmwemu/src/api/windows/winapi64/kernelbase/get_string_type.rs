use crate::emu;

pub fn GetStringTypeA(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let info_type = emu.regs().rdx;
    let src_ptr = emu.regs().r8;
    let src_len = emu.regs().r9;
    let src = if src_ptr != 0 {
        emu.maps.read_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!GetStringTypeA type:0x{:x} '{}'",
        info_type,
        src
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetStringTypeW(emu: &mut emu::Emu) {
    let info_type = emu.regs().rcx as u32;
    let src_ptr = emu.regs().rdx;
    let src_len = emu.regs().r8 as i32;
    let out_ptr = emu.regs().r9;
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let char_count = if src_len < 0 {
        src.encode_utf16().count()
    } else {
        src_len as usize
    };
    if out_ptr != 0 {
        for (i, ch) in src.chars().take(char_count).enumerate() {
            let ct: u16 = if info_type == 1 {
                let mut t: u16 = 0;
                if ch.is_uppercase() {
                    t |= 0x0001;
                }
                if ch.is_lowercase() {
                    t |= 0x0002;
                }
                if ch.is_ascii_digit() {
                    t |= 0x0004;
                }
                if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                    t |= 0x0008;
                }
                if ch.is_ascii_punctuation() {
                    t |= 0x0010;
                }
                if ch.is_control() {
                    t |= 0x0020;
                }
                if ch == ' ' {
                    t |= 0x0048;
                }
                if ch.is_alphanumeric() {
                    t |= 0x0100;
                }
                t
            } else {
                0
            };
            emu.maps.write_word(out_ptr + (i as u64) * 2, ct);
        }
    }
    log_red!(
        emu,
        "kernelbase!GetStringTypeW type:0x{:x} '{}' len:{}",
        info_type,
        src,
        char_count
    );
    emu.regs_mut().rax = 1;
}
