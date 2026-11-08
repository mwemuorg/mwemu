use crate::emu;

pub fn GetStringTypeA(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let dw_info_type = emu.regs().rdx as u32;
    let lp_src_str = emu.regs().r8;
    let cch_src = emu.regs().r9 as i32;
    let lp_char_type = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let actual_len = if cch_src == -1 {
        let s = emu.maps.read_string(lp_src_str);
        s.len() + 1
    } else {
        cch_src as usize
    };

    if lp_char_type != 0 && actual_len > 0 {
        for i in 0..actual_len {
            let ch = emu.maps.read_byte(lp_src_str + i as u64).unwrap_or(0) as char;
            let ct: u16 = match dw_info_type {
                1 => {
                    let mut t: u16 = 0;
                    if ch.is_ascii_uppercase() {
                        t |= 0x0001;
                    }
                    if ch.is_ascii_lowercase() {
                        t |= 0x0002;
                    }
                    if ch.is_ascii_digit() {
                        t |= 0x0004;
                    }
                    if ch == ' ' || ch == '\t' {
                        t |= 0x0008;
                    }
                    if ch.is_ascii_punctuation() {
                        t |= 0x0010;
                    }
                    if (ch as u32) < 0x20 || ch as u32 == 0x7f {
                        t |= 0x0020;
                    }
                    if ch.is_ascii_alphanumeric() {
                        t |= 0x0100;
                    }
                    t
                }
                _ => 0,
            };
            emu.maps.write_word(lp_char_type + (i as u64) * 2, ct);
        }
    }

    log_red!(
        emu,
        "kernel32!GetStringTypeA locale:{} type:{} len:{} => TRUE",
        locale,
        dw_info_type,
        actual_len
    );
    emu.regs_mut().rax = 1;
}
