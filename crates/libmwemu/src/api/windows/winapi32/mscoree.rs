use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "_CorExeMain" => _CorExeMain(emu),

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!(
                "attempt to call unimplemented mscoree API 0x{:x} {}",
                addr,
                api
            );
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

pub fn _CorExeMain(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "mscoree!_CorExeMain  .NET CLR entry — parsing metadata"
    );
    parse_dotnet_metadata(emu);
    emu.regs_mut().rax = 0;
    emu.stop();
}

fn parse_dotnet_metadata(emu: &mut emu::Emu) {
    let base = emu
        .pe32
        .as_ref()
        .map(|pe| pe.opt.image_base as u64)
        .unwrap_or(emu.cfg.code_base_addr);

    let pe_off = match emu.maps.read_dword(base + 0x3c) {
        Some(v) => v as u64,
        None => return,
    };

    let clr_dd_off = base + pe_off + 0x18 + 0x60 + 14 * 8;
    let clr_rva = match emu.maps.read_dword(clr_dd_off) {
        Some(v) => v as u64,
        None => return,
    };
    let clr_size = emu.maps.read_dword(clr_dd_off + 4).unwrap_or(0) as u64;

    if clr_rva == 0 || clr_size < 0x48 {
        log_red!(emu, "\tno CLR header found");
        return;
    }

    let clr_addr = base + clr_rva;
    let major = emu.maps.read_word(clr_addr + 4).unwrap_or(0);
    let minor = emu.maps.read_word(clr_addr + 6).unwrap_or(0);
    let meta_rva = emu.maps.read_dword(clr_addr + 8).unwrap_or(0) as u64;
    let meta_size = emu.maps.read_dword(clr_addr + 12).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(clr_addr + 16).unwrap_or(0);
    let entry_token = emu.maps.read_dword(clr_addr + 20).unwrap_or(0);
    let res_rva = emu.maps.read_dword(clr_addr + 24).unwrap_or(0) as u64;
    let res_size = emu.maps.read_dword(clr_addr + 28).unwrap_or(0) as u64;

    log_red!(emu, "\t--- .NET CLR Header ---");
    log_red!(emu, "\tRuntime: {}.{}", major, minor);
    log_red!(
        emu,
        "\tFlags: 0x{:x}{}",
        flags,
        if flags & 1 != 0 { " (IL_ONLY)" } else { "" }
    );
    log_red!(emu, "\tEntryPoint token: 0x{:08x}", entry_token);
    log_red!(
        emu,
        "\tMetadata: RVA=0x{:x} Size=0x{:x}",
        meta_rva,
        meta_size
    );
    if res_rva != 0 {
        log_red!(
            emu,
            "\tResources: RVA=0x{:x} Size=0x{:x}",
            res_rva,
            res_size
        );
    }

    if meta_rva == 0 || meta_size == 0 {
        return;
    }

    let meta_addr = base + meta_rva;
    let sig = emu.maps.read_dword(meta_addr).unwrap_or(0);
    if sig != 0x424A5342 {
        log_red!(emu, "\tinvalid metadata signature: 0x{:08x}", sig);
        return;
    }

    let ver_len = emu.maps.read_dword(meta_addr + 12).unwrap_or(0) as u64;
    if ver_len > 0 && ver_len < 256 {
        let ver_bytes = emu.maps.read_bytes(meta_addr + 16, ver_len as usize);
        let ver = String::from_utf8_lossy(ver_bytes)
            .trim_end_matches('\0')
            .to_string();
        log_red!(emu, "\tFramework: {}", ver);
    }

    let streams_base = (meta_addr + 16 + ver_len + 3) & !3;
    let num_streams = emu.maps.read_word(streams_base + 2).unwrap_or(0);

    log_red!(emu, "\t--- Streams ({}) ---", num_streams);

    let mut pos = streams_base + 4;
    let mut us_offset: u64 = 0;
    let mut us_size: u64 = 0;
    let mut strings_offset: u64 = 0;
    let mut strings_size: u64 = 0;

    for _ in 0..num_streams {
        let s_off = emu.maps.read_dword(pos).unwrap_or(0) as u64;
        let s_sz = emu.maps.read_dword(pos + 4).unwrap_or(0) as u64;
        pos += 8;

        let mut name_bytes = Vec::new();
        for j in 0..32u64 {
            let b = emu.maps.read_byte(pos + j).unwrap_or(0);
            if b == 0 {
                pos += j + 1;
                break;
            }
            name_bytes.push(b);
        }
        pos = (pos + 3) & !3;

        let name = String::from_utf8_lossy(&name_bytes).to_string();
        log_red!(emu, "\t  {}: offset=0x{:x} size=0x{:x}", name, s_off, s_sz);

        if name == "#US" {
            us_offset = s_off;
            us_size = s_sz;
        } else if name == "#Strings" {
            strings_offset = s_off;
            strings_size = s_sz;
        }
    }

    if strings_size > 1 {
        log_red!(emu, "\t--- #Strings ---");
        let str_addr = meta_addr + strings_offset;
        let mut spos: u64 = 1;
        let mut count = 0;
        while spos < strings_size && count < 50 {
            let mut s = Vec::new();
            while spos < strings_size {
                let b = emu.maps.read_byte(str_addr + spos).unwrap_or(0);
                spos += 1;
                if b == 0 {
                    break;
                }
                s.push(b);
            }
            if !s.is_empty() {
                let text = String::from_utf8_lossy(&s);
                if text.len() >= 2 {
                    log_red!(emu, "\t  {}", text);
                    count += 1;
                }
            }
        }
        if spos < strings_size {
            log_red!(emu, "\t  ... ({} more bytes)", strings_size - spos);
        }
    }

    if us_size > 1 {
        log_red!(emu, "\t--- #US (User Strings) ---");
        let us_addr = meta_addr + us_offset;
        let mut spos: u64 = 1;
        let mut count = 0;
        while spos < us_size && count < 50 {
            let b = emu.maps.read_byte(us_addr + spos).unwrap_or(0);
            if b == 0 {
                break;
            }
            let slen: u64;
            if b < 0x80 {
                slen = b as u64;
                spos += 1;
            } else if b < 0xC0 {
                let b2 = emu.maps.read_byte(us_addr + spos + 1).unwrap_or(0);
                slen = (((b & 0x3f) as u64) << 8) | b2 as u64;
                spos += 2;
            } else {
                let b2 = emu.maps.read_byte(us_addr + spos + 1).unwrap_or(0);
                let b3 = emu.maps.read_byte(us_addr + spos + 2).unwrap_or(0);
                let b4 = emu.maps.read_byte(us_addr + spos + 3).unwrap_or(0);
                slen = (((b & 0x1f) as u64) << 24)
                    | ((b2 as u64) << 16)
                    | ((b3 as u64) << 8)
                    | b4 as u64;
                spos += 4;
            }
            if slen == 0 || spos + slen > us_size {
                break;
            }
            let raw = emu.maps.read_bytes(us_addr + spos, slen as usize);
            spos += slen;

            let mut chars = Vec::new();
            for chunk in raw.chunks(2) {
                if chunk.len() == 2 {
                    let c = u16::from_le_bytes([chunk[0], chunk[1]]);
                    if c != 0 {
                        chars.push(c);
                    }
                }
            }
            if !chars.is_empty() {
                let text = String::from_utf16_lossy(&chars);
                if text.len() >= 2
                    && text
                        .chars()
                        .all(|c| !c.is_control() || c == '\t' || c == '\n')
                {
                    log_red!(emu, "\t  \"{}\"", text);
                    count += 1;
                }
            }
        }
        if count == 50 {
            log_red!(emu, "\t  ... (truncated)");
        }
    }

    log_red!(emu, "\t--- end .NET metadata ---");
}
