use goblin::pe::{
    data_directories::SIZEOF_DATA_DIRECTORY,
    header::{DOS_MAGIC, PE_MAGIC, PE_POINTER_OFFSET, SIZEOF_COFF_HEADER, SIZEOF_PE_MAGIC},
    optional_header::{
        MAGIC_32, MAGIC_64, SIZEOF_STANDARD_FIELDS_32, SIZEOF_STANDARD_FIELDS_64,
        SIZEOF_WINDOWS_FIELDS_32, SIZEOF_WINDOWS_FIELDS_64,
    },
};
use rs_header::pe::shared::IMAGE_DIRECTORY_ENTRY_DOTNET_HDR;

use crate::emu;
use crate::winapi::common::memory::read_guest_bytes;
use crate::winapi::policy;

pub(crate) fn entry(emu: &mut emu::Emu) {
    log_red!(emu, "mscoree!_CorExeMain inspecting CLR metadata");
    parse_dotnet_metadata(emu);
    policy::unsupported(emu, "mscoree", "_CorExeMain", emu.pc());
}

const COR20_HEADER_SIZE: u64 = 72;
const OPTIONAL_HEADER_IMAGE_SIZE_OFFSET: u64 = 56;

/// Return the mapped CLR header and the exclusive end of its PE image.
pub(crate) fn clr_header(emu: &emu::Emu) -> Option<(u64, u64)> {
    let base = emu.base;
    if emu.maps.read_word(base)? != DOS_MAGIC {
        return None;
    }
    let pe = base.checked_add(u64::from(
        emu.maps
            .read_dword(base.checked_add(PE_POINTER_OFFSET.into())?)?,
    ))?;
    if emu.maps.read_dword(pe)? != PE_MAGIC {
        return None;
    }
    let optional = pe.checked_add((SIZEOF_PE_MAGIC + SIZEOF_COFF_HEADER) as u64)?;
    // The final two COFF fields are SizeOfOptionalHeader and Characteristics.
    let optional_size = emu.maps.read_word(optional - 4)? as usize;
    let dirs = match emu.maps.read_word(optional)? {
        MAGIC_32 => SIZEOF_STANDARD_FIELDS_32 + SIZEOF_WINDOWS_FIELDS_32,
        MAGIC_64 => SIZEOF_STANDARD_FIELDS_64 + SIZEOF_WINDOWS_FIELDS_64,
        _ => return None,
    };
    let offset = dirs + IMAGE_DIRECTORY_ENTRY_DOTNET_HDR * SIZEOF_DATA_DIRECTORY;
    if optional_size < offset + SIZEOF_DATA_DIRECTORY
        || emu
            .maps
            .read_dword(optional.checked_add((dirs - 4) as u64)?)? as usize
            <= IMAGE_DIRECTORY_ENTRY_DOTNET_HDR
    {
        return None;
    }
    let addr = optional.checked_add(offset as u64)?;
    let rva = u64::from(emu.maps.read_dword(addr)?);
    let size = u64::from(emu.maps.read_dword(addr.checked_add(4)?)?);
    let image_size = u64::from(
        emu.maps
            .read_dword(optional.checked_add(OPTIONAL_HEADER_IMAGE_SIZE_OFFSET)?)?,
    );
    if rva == 0 || size < COR20_HEADER_SIZE || rva + size > image_size {
        return None;
    }
    Some((base.checked_add(rva)?, base.checked_add(image_size)?))
}

fn parse_dotnet_metadata(emu: &mut emu::Emu) {
    let base = emu.base;
    let Some((clr_addr, image_end)) = clr_header(emu) else {
        log_red!(emu, "\tno CLR header found");
        return;
    };
    let major = emu.maps.read_word(clr_addr + 4).unwrap_or(0);
    let minor = emu.maps.read_word(clr_addr + 6).unwrap_or(0);
    let meta_rva = emu.maps.read_dword(clr_addr + 8).unwrap_or(0) as u64;
    let meta_size = emu.maps.read_dword(clr_addr + 12).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(clr_addr + 16).unwrap_or(0);
    let entry_token = emu.maps.read_dword(clr_addr + 20).unwrap_or(0);
    let res_rva = emu.maps.read_dword(clr_addr + 24).unwrap_or(0) as u64;
    let res_size = emu.maps.read_dword(clr_addr + 28).unwrap_or(0) as u64;

    log_red!(emu, "\t--- .NET CLR Header ---");
    log_red!(emu, "\tRuntime={}.{}", major, minor);
    log_red!(
        emu,
        "\tFlags=0x{:x}{}",
        flags,
        if flags & 1 != 0 { " (IL_ONLY)" } else { "" }
    );
    log_red!(emu, "\tEntryPoint token=0x{:08x}", entry_token);
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

    let Some(meta_addr) = base.checked_add(meta_rva) else {
        return;
    };
    let Some(meta_end) = meta_addr.checked_add(meta_size) else {
        return;
    };
    if meta_size < 20 || meta_end > image_end {
        return;
    }
    let sig = emu.maps.read_dword(meta_addr).unwrap_or(0);
    if sig != 0x424A5342 {
        log_red!(emu, "\tinvalid metadata signature=0x{:08x}", sig);
        return;
    }

    let ver_len = emu.maps.read_dword(meta_addr + 12).unwrap_or(0) as u64;
    if ver_len > meta_size - 20 {
        return;
    }
    if ver_len > 0 && ver_len < 256 {
        let Some(ver_bytes) = read_guest_bytes(emu, meta_addr + 16, ver_len as usize) else {
            return;
        };
        let ver = String::from_utf8_lossy(&ver_bytes)
            .trim_end_matches('\0')
            .to_string();
        log_red!(emu, "\tFramework={}", ver);
    }

    let Some(streams_base) = (meta_addr + 16 + ver_len).checked_next_multiple_of(4) else {
        return;
    };
    if meta_end.saturating_sub(streams_base) < 4 {
        return;
    }
    let num_streams = emu.maps.read_word(streams_base + 2).unwrap_or(0);

    log_red!(emu, "\t--- Streams ({}) ---", num_streams);

    let mut pos = streams_base + 4;
    let mut us_offset: u64 = 0;
    let mut us_size: u64 = 0;
    let mut strings_offset: u64 = 0;
    let mut strings_size: u64 = 0;

    const MAX_STREAM_NAME: u64 = 32;
    const MAX_PREVIEW_BYTES: u64 = 65536;
    for _ in 0..num_streams {
        if meta_end.saturating_sub(pos) < 9 {
            return;
        }
        let s_off = emu.maps.read_dword(pos).unwrap_or(0) as u64;
        let s_sz = emu.maps.read_dword(pos + 4).unwrap_or(0) as u64;
        pos += 8;

        let mut name_bytes = Vec::new();
        loop {
            if pos >= meta_end || name_bytes.len() as u64 > MAX_STREAM_NAME {
                return;
            }
            let Some(b) = emu.maps.read_byte(pos) else {
                return;
            };
            pos += 1;
            if b == 0 {
                break;
            }
            name_bytes.push(b);
        }
        let Some(next) = pos.checked_next_multiple_of(4) else {
            return;
        };
        pos = next;
        if s_off > meta_size || s_sz > meta_size - s_off {
            return;
        }
        let name = String::from_utf8_lossy(&name_bytes).to_string();
        log_red!(emu, "\t  {}: offset=0x{:x} size=0x{:x}", name, s_off, s_sz);

        if name == "#US" {
            us_offset = s_off;
            us_size = s_sz.min(MAX_PREVIEW_BYTES);
        } else if name == "#Strings" {
            strings_offset = s_off;
            strings_size = s_sz.min(MAX_PREVIEW_BYTES);
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
                if us_size - spos < 2 {
                    break;
                }
                let b2 = emu.maps.read_byte(us_addr + spos + 1).unwrap_or(0);
                slen = (((b & 0x3f) as u64) << 8) | b2 as u64;
                spos += 2;
            } else {
                if b >= 0xe0 || us_size - spos < 4 {
                    break;
                }
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
            let Some(raw) = read_guest_bytes(emu, us_addr + spos, slen as usize) else {
                return;
            };
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
