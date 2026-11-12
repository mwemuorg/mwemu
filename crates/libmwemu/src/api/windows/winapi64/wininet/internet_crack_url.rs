use crate::emu;

pub(super) fn InternetCrackUrlA(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let url_len = emu.regs().rdx as u32;
    let _flags = emu.regs().r8;
    let components_ptr = emu.regs().r9;

    let url = if url_ptr != 0 {
        if url_len == 0 {
            emu.maps.read_string(url_ptr)
        } else {
            emu.maps.read_string_of_bytes(url_ptr, url_len as usize)
        }
    } else {
        String::new()
    };

    log_red!(emu, "wininet!InternetCrackUrlA url: '{}'", url);

    if components_ptr == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    // Parse simple URL: scheme://host:port/path
    let (scheme, rest) = if let Some(pos) = url.find("://") {
        (&url[..pos], &url[pos + 3..])
    } else {
        ("http", url.as_str())
    };

    let (host_port, path) = if let Some(pos) = rest.find('/') {
        (&rest[..pos], &rest[pos..])
    } else {
        (rest, "/")
    };

    let (host, port) = if let Some(pos) = host_port.find(':') {
        (
            &host_port[..pos],
            host_port[pos + 1..].parse::<u16>().unwrap_or(80),
        )
    } else {
        (
            host_port,
            match scheme {
                "https" => 443u16,
                "ftp" => 21,
                _ => 80,
            },
        )
    };

    let scheme_id: u32 = match scheme {
        "https" => 2,
        "ftp" => 3,
        _ => 1, // INTERNET_SCHEME_HTTP
    };

    // URL_COMPONENTSA struct layout (64-bit):
    // 0x08: lpszScheme + 0x10: dwSchemeLength
    // 0x18: nScheme
    // 0x20: lpszHostName + 0x28: dwHostNameLength
    // 0x30: nPort
    // 0x58: lpszUrlPath + 0x60: dwUrlPathLength
    let scheme_buf = emu.maps.read_qword(components_ptr + 0x08).unwrap_or(0);
    let host_buf = emu.maps.read_qword(components_ptr + 0x20).unwrap_or(0);
    let path_buf = emu.maps.read_qword(components_ptr + 0x58).unwrap_or(0);

    if scheme_buf != 0 {
        emu.maps.write_string(scheme_buf, scheme);
        emu.maps
            .write_dword(components_ptr + 0x10, scheme.len() as u32);
    }
    emu.maps.write_dword(components_ptr + 0x18, scheme_id);

    if host_buf != 0 {
        emu.maps.write_string(host_buf, host);
        emu.maps
            .write_dword(components_ptr + 0x28, host.len() as u32);
    }
    emu.maps.write_word(components_ptr + 0x30, port);

    if path_buf != 0 {
        emu.maps.write_string(path_buf, path);
        emu.maps
            .write_dword(components_ptr + 0x60, path.len() as u32);
    }

    emu.regs_mut().rax = 1; // TRUE
}

pub(super) fn InternetCrackUrlW(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let _url_len = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let components_ptr = emu.regs().r9;

    let url = if url_ptr != 0 {
        emu.maps.read_wide_string(url_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "wininet!InternetCrackUrlW url: '{}'", url);

    if components_ptr == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let (scheme, rest) = if let Some(pos) = url.find("://") {
        (&url[..pos], &url[pos + 3..])
    } else {
        ("http", url.as_str())
    };

    let (host_port, path) = if let Some(pos) = rest.find('/') {
        (&rest[..pos], &rest[pos..])
    } else {
        (rest, "/")
    };

    let (host, port) = if let Some(pos) = host_port.find(':') {
        (
            &host_port[..pos],
            host_port[pos + 1..].parse::<u16>().unwrap_or(80),
        )
    } else {
        (
            host_port,
            match scheme {
                "https" => 443u16,
                "ftp" => 21,
                _ => 80,
            },
        )
    };

    let scheme_id: u32 = match scheme {
        "https" => 2,
        "ftp" => 3,
        _ => 1,
    };

    let scheme_buf = emu.maps.read_qword(components_ptr + 0x08).unwrap_or(0);
    let host_buf = emu.maps.read_qword(components_ptr + 0x20).unwrap_or(0);
    let path_buf = emu.maps.read_qword(components_ptr + 0x58).unwrap_or(0);

    if scheme_buf != 0 {
        emu.maps.write_wide_string(scheme_buf, scheme);
        emu.maps
            .write_dword(components_ptr + 0x10, scheme.len() as u32);
    }
    emu.maps.write_dword(components_ptr + 0x18, scheme_id);

    if host_buf != 0 {
        emu.maps.write_wide_string(host_buf, host);
        emu.maps
            .write_dword(components_ptr + 0x28, host.len() as u32);
    }
    emu.maps.write_word(components_ptr + 0x30, port);

    if path_buf != 0 {
        emu.maps.write_wide_string(path_buf, path);
        emu.maps
            .write_dword(components_ptr + 0x60, path.len() as u32);
    }

    emu.regs_mut().rax = 1;
}
