use crate::{emu, windows::constants};

pub fn MultiByteToWideChar(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx;
    let dw_flags = emu.regs().rdx;
    let utf8_ptr = emu.regs().r8;
    let cb_multi_byte = emu.regs().r9 as i64;
    let wide_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let cch_wide_char = emu
        .maps
        .read_dword(emu.regs().rsp + 0x28) // yes, read only half of the stack item
        .unwrap_or(0) as i32;

    let raw_bytes: Vec<u8> = if utf8_ptr > 0 {
        if cb_multi_byte == -1 {
            let s = emu.maps.read_string(utf8_ptr);
            s.into_bytes()
        } else if (0..=10_000_000).contains(&cb_multi_byte) {
            emu.maps
                .read_bytes(utf8_ptr, cb_multi_byte as usize)
                .to_vec()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    log_red!(
        emu,
        ":{:x} kernel32!MultiByteToWideChar code_page: {} dw_flags: {} utf8_ptr: 0x{:x} cb_multi_byte: {} wide_ptr: 0x{:x} cch_wide_char: {}",
        emu.regs().rip,
        code_page,
        dw_flags,
        utf8_ptr,
        cb_multi_byte,
        wide_ptr,
        cch_wide_char
    );

    let wide: Vec<u16> = if code_page == 65001 {
        // CP_UTF8: real UTF-8 decoding
        let s = String::from_utf8_lossy(&raw_bytes);
        let mut v: Vec<u16> = s.encode_utf16().collect();
        if cb_multi_byte == -1 {
            v.push(0);
        }
        v
    } else {
        // Single-byte code pages (1252, 437, etc.): each byte maps to one u16
        // using the CP1252 table for bytes 0x80-0x9F, otherwise byte == codepoint
        let mut v: Vec<u16> = raw_bytes.iter().map(|&b| cp1252_to_unicode(b)).collect();
        if cb_multi_byte == -1 {
            v.push(0);
        }
        v
    };

    if cch_wide_char == 0 {
        // Return required buffer size for UTF-16 conversion
        emu.regs_mut().rax = wide.len() as u64;
    } else if wide_ptr != 0 {
        // Write string if buffer is large enough
        if cch_wide_char >= wide.len() as i32 {
            for (i, wchar) in wide.iter().enumerate() {
                emu.maps.write_word(wide_ptr + (i * 2) as u64, *wchar);
            }
            emu.regs_mut().rax = wide.len() as u64;
        } else {
            emu.set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = 0;
    }
}

fn cp1252_to_unicode(b: u8) -> u16 {
    // Bytes 0x80-0x9F differ between CP1252 and Unicode; the rest map 1:1
    match b {
        0x80 => 0x20AC, // €
        0x82 => 0x201A, // ‚
        0x83 => 0x0192, // ƒ
        0x84 => 0x201E, // „
        0x85 => 0x2026, // …
        0x86 => 0x2020, // †
        0x87 => 0x2021, // ‡
        0x88 => 0x02C6, // ˆ
        0x89 => 0x2030, // ‰
        0x8A => 0x0160, // Š
        0x8B => 0x2039, // ‹
        0x8C => 0x0152, // Œ
        0x8E => 0x017D, // Ž
        0x91 => 0x2018, // '
        0x92 => 0x2019, // '
        0x93 => 0x201C, // "
        0x94 => 0x201D, // "
        0x95 => 0x2022, // •
        0x96 => 0x2013, // –
        0x97 => 0x2014, // —
        0x98 => 0x02DC, // ˜
        0x99 => 0x2122, // ™
        0x9A => 0x0161, // š
        0x9B => 0x203A, // ›
        0x9C => 0x0153, // œ
        0x9E => 0x017E, // ž
        0x9F => 0x0178, // Ÿ
        _ => b as u16,
    }
}
