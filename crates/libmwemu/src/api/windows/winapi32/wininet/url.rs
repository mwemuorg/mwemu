use crate::emu;

use super::{pop_stack32, read_stack_dword};

pub(super) fn internet_crack_url_a(emu: &mut emu::Emu) {
    let url_ptr =
        read_stack_dword(emu, 0, "wininet!InternetCrackUrlA error reading url_ptr") as u64;
    let url_len = read_stack_dword(emu, 4, "wininet!InternetCrackUrlA error reading url_len");
    let _flags = read_stack_dword(emu, 8, "wininet!InternetCrackUrlA error reading flags");
    let components_ptr = read_stack_dword(
        emu,
        12,
        "wininet!InternetCrackUrlA error reading components",
    ) as u64;

    let url = if url_ptr != 0 {
        if url_len == 0 {
            emu.maps.read_string(url_ptr)
        } else {
            emu.maps.read_string_of_bytes(url_ptr, url_len as usize)
        }
    } else {
        String::new()
    };

    log_red!(emu, "wininet!InternetCrackUrlA url: `{}`", url);

    pop_stack32(emu, 4);

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

    // URL_COMPONENTSA struct layout (32-bit):
    // 0x04: lpszScheme + 0x08: dwSchemeLength
    // 0x0C: nScheme
    // 0x10: lpszHostName + 0x14: dwHostNameLength
    // 0x18: nPort
    // 0x28: lpszUrlPath + 0x2C: dwUrlPathLength
    let scheme_buf = emu.maps.read_dword(components_ptr + 0x04).unwrap_or(0) as u64;
    let host_buf = emu.maps.read_dword(components_ptr + 0x10).unwrap_or(0) as u64;
    let path_buf = emu.maps.read_dword(components_ptr + 0x28).unwrap_or(0) as u64;

    if scheme_buf != 0 {
        emu.maps.write_string(scheme_buf, scheme);
        emu.maps
            .write_dword(components_ptr + 0x08, scheme.len() as u32);
    }
    emu.maps.write_dword(components_ptr + 0x0C, scheme_id);

    if host_buf != 0 {
        emu.maps.write_string(host_buf, host);
        emu.maps
            .write_dword(components_ptr + 0x14, host.len() as u32);
    }
    emu.maps.write_word(components_ptr + 0x18, port);

    if path_buf != 0 {
        emu.maps.write_string(path_buf, path);
        emu.maps
            .write_dword(components_ptr + 0x2C, path.len() as u32);
    }

    emu.regs_mut().rax = 1;
}

pub(super) fn internet_crack_url_w(emu: &mut emu::Emu) {
    let url_ptr =
        read_stack_dword(emu, 0, "wininet!InternetCrackUrlW error reading url_ptr") as u64;
    let _url_len = read_stack_dword(emu, 4, "wininet!InternetCrackUrlW error reading url_len");
    let _flags = read_stack_dword(emu, 8, "wininet!InternetCrackUrlW error reading flags");
    let components_ptr = read_stack_dword(
        emu,
        12,
        "wininet!InternetCrackUrlW error reading components",
    ) as u64;

    let url = if url_ptr != 0 {
        emu.maps.read_wide_string(url_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "wininet!InternetCrackUrlW url: `{}`", url);

    pop_stack32(emu, 4);

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

    let scheme_buf = emu.maps.read_dword(components_ptr + 0x04).unwrap_or(0) as u64;
    let host_buf = emu.maps.read_dword(components_ptr + 0x10).unwrap_or(0) as u64;
    let path_buf = emu.maps.read_dword(components_ptr + 0x28).unwrap_or(0) as u64;

    if scheme_buf != 0 {
        emu.maps.write_wide_string(scheme_buf, scheme);
        emu.maps
            .write_dword(components_ptr + 0x08, scheme.len() as u32);
    }
    emu.maps.write_dword(components_ptr + 0x0C, scheme_id);

    if host_buf != 0 {
        emu.maps.write_wide_string(host_buf, host);
        emu.maps
            .write_dword(components_ptr + 0x14, host.len() as u32);
    }
    emu.maps.write_word(components_ptr + 0x18, port);

    if path_buf != 0 {
        emu.maps.write_wide_string(path_buf, path);
        emu.maps
            .write_dword(components_ptr + 0x2C, path.len() as u32);
    }

    emu.regs_mut().rax = 1;
}
