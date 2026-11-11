use crate::emu;

pub(super) fn HttpQueryInfoA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let info_level = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    let buffer_len_ptr = emu.regs().r9;
    let level_raw = info_level & 0x0000FFFF;

    log_red!(
        emu,
        "wininet!HttpQueryInfoA handle: 0x{:x} level: 0x{:x} (raw: {})",
        handle,
        info_level,
        level_raw
    );

    let response: Option<&str> = match level_raw {
        1 => Some("text/html"),            // HTTP_QUERY_CONTENT_TYPE
        5 => Some("1024"),                 // HTTP_QUERY_CONTENT_LENGTH
        19 => Some("200"),                 // HTTP_QUERY_STATUS_CODE
        21 => Some("HTTP/1.1 200 OK"),     // HTTP_QUERY_RAW_HEADERS
        22 => Some("HTTP/1.1 200 OK\r\n"), // HTTP_QUERY_RAW_HEADERS_CRLF
        _ => None,
    };

    match response {
        Some(val) => {
            if buffer != 0 {
                emu.maps.write_string(buffer, val);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, val.len() as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        None => {
            log_red!(emu, "  unsupported info level {}", level_raw);
            emu.regs_mut().rax = 0; // FALSE
        }
    }
}

pub(super) fn HttpQueryInfoW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let info_level = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    let buffer_len_ptr = emu.regs().r9;
    let level_raw = info_level & 0x0000FFFF;

    log_red!(
        emu,
        "wininet!HttpQueryInfoW handle: 0x{:x} level: 0x{:x} (raw: {})",
        handle,
        info_level,
        level_raw
    );

    let response: Option<&str> = match level_raw {
        1 => Some("text/html"),
        5 => Some("1024"),
        19 => Some("200"),
        21 => Some("HTTP/1.1 200 OK"),
        22 => Some("HTTP/1.1 200 OK\r\n"),
        _ => None,
    };

    match response {
        Some(val) => {
            if buffer != 0 {
                emu.maps.write_wide_string(buffer, val);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, (val.len() * 2) as u32);
            }
            emu.regs_mut().rax = 1;
        }
        None => {
            emu.regs_mut().rax = 0;
        }
    }
}
