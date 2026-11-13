//! The runtime library: counted strings, security descriptors, version
//! queries, registry helpers, bitmaps and image helpers.
//!
//! The security-descriptor group matters more than it looks: `wdmsec.lib`'s
//! `IoCreateDeviceSecure` is statically linked into most vendor drivers and
//! builds a DACL from `SeExports` SIDs through exactly these calls before it
//! reaches `IoCreateDevice`. If they returned garbage, DriverEntry would fail
//! before the device the exploit talks to ever existed.

use super::structs::*;
use super::*;
use crate::kernel::heap::Region;

fn unicode_len(emu: &Emu, us: u64) -> u64 {
    emu.maps.read_word(us).unwrap_or(0) as u64
}

fn unicode_max(emu: &Emu, us: u64) -> u64 {
    emu.maps.read_word(us + 2).unwrap_or(0) as u64
}

fn unicode_buf(emu: &Emu, us: u64) -> u64 {
    emu.maps.read_qword(us + 8).unwrap_or(0)
}

/// Append `bytes` (UTF-16LE) to the counted string at `dst`.
fn unicode_append(emu: &mut Emu, dst: u64, bytes: &[u8]) -> u64 {
    let len = unicode_len(emu, dst);
    let max = unicode_max(emu, dst);
    let buf = unicode_buf(emu, dst);
    if buf == 0 || len + bytes.len() as u64 > max {
        return STATUS_BUFFER_TOO_SMALL;
    }
    let rip = emu.pc();
    emu.kernel_guard_access(rip, buf + len, bytes.len() as u32, true);
    emu.maps.write_bytes(buf + len, bytes);
    let new_len = len + bytes.len() as u64;
    emu.maps.write_word(dst, new_len as u16);
    if new_len + 2 <= max {
        emu.maps.write_word(buf + new_len, 0);
    }
    STATUS_SUCCESS
}

fn utf16_bytes(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(|c| c.to_le_bytes()).collect()
}

fn compare_strings(a: &str, b: &str, case_insensitive: bool) -> i64 {
    let (x, y) = if case_insensitive {
        (a.to_lowercase(), b.to_lowercase())
    } else {
        (a.to_string(), b.to_string())
    };
    match x.cmp(&y) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// `RtlLengthSid`: `8 + 4 * SubAuthorityCount`.
fn sid_length(emu: &Emu, sid: u64) -> u64 {
    if sid == 0 || !emu.maps.is_mapped(sid) {
        return 0;
    }
    8 + 4 * emu.maps.read_byte(sid + 1).unwrap_or(0) as u64
}

/// Append an ACE of `ace_type` for `sid` to the ACL at `acl`.
fn add_ace(emu: &mut Emu, acl: u64, ace_type: u8, mask: u32, sid: u64) -> u64 {
    let acl_size = emu.maps.read_word(acl + ACL_SIZE).unwrap_or(0) as u64;
    let count = emu.maps.read_word(acl + ACL_ACE_COUNT).unwrap_or(0);
    let sid_len = sid_length(emu, sid);
    // Walk existing ACEs to find the end.
    let mut off = ACL_HEADER_SIZE;
    for _ in 0..count {
        let sz = emu.maps.read_word(acl + off + 2).unwrap_or(0) as u64;
        if sz == 0 {
            break;
        }
        off += sz;
    }
    let ace_size = ACE_HEADER_SIZE + sid_len;
    if off + ace_size > acl_size {
        return STATUS_BUFFER_TOO_SMALL;
    }
    let ace = acl + off;
    emu.maps.write_byte(ace, ace_type);
    emu.maps.write_byte(ace + 1, 0);
    emu.maps.write_word(ace + 2, ace_size as u16);
    emu.maps.write_dword(ace + 4, mask);
    if sid_len > 0 {
        emu.maps.memcpy(ace + 8, sid, sid_len as usize);
    }
    emu.maps.write_word(acl + ACL_ACE_COUNT, count + 1);
    STATUS_SUCCESS
}

/// Total size of a self-relative image of the absolute SD at `sd`.
fn sd_self_relative_size(emu: &Emu, sd: u64) -> u64 {
    let mut total = SD_ABSOLUTE_SIZE;
    for part in [SD_OWNER, SD_GROUP] {
        let p = emu.maps.read_qword(sd + part).unwrap_or(0);
        total += sid_length(emu, p);
    }
    for part in [SD_SACL, SD_DACL] {
        let p = emu.maps.read_qword(sd + part).unwrap_or(0);
        if p != 0 && emu.maps.is_mapped(p) {
            total += emu.maps.read_word(p + ACL_SIZE).unwrap_or(0) as u64;
        }
    }
    total
}

/// RTL_OSVERSIONINFOW / EXW: Windows 10 22H2 workstation.
fn fill_version_info(emu: &mut Emu, info: u64) {
    let size = emu.maps.read_dword(info).unwrap_or(0x114);
    emu.maps.write_dword(info + 4, OS_MAJOR);
    emu.maps.write_dword(info + 8, OS_MINOR);
    emu.maps.write_dword(info + 0xc, NT_BUILD_NUMBER as u32);
    emu.maps.write_dword(info + 0x10, 2); // VER_PLATFORM_WIN32_NT
    emu.maps.write_word(info + 0x14, 0); // szCSDVersion = L""
    if size >= 0x11c {
        emu.maps.write_word(info + 0x114, 0); // wServicePackMajor
        emu.maps.write_word(info + 0x116, 0);
        emu.maps.write_word(info + 0x118, 0x100); // wSuiteMask: VER_SUITE_SINGLEUSERTS
        emu.maps.write_byte(info + 0x11a, 1); // VER_NT_WORKSTATION
    }
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- memory helpers (guarded) ------------------------------------------
        "RtlCopyMemory" | "RtlMoveMemory" | "RtlCopyMemoryNonTemporal" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            crt::checked_copy(emu, dst, src, len);
            emu.set_kernel_ret(dst);
        }
        "RtlZeroMemory" | "RtlSecureZeroMemory" => {
            let dst = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            crt::checked_fill(emu, dst, 0, len);
            emu.set_kernel_ret(dst);
        }
        "RtlFillMemory" => {
            let dst = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let byte = emu.kernel_arg(2) as u8;
            crt::checked_fill(emu, dst, byte, len);
            emu.set_kernel_ret(dst);
        }
        "RtlCompareMemory" => {
            let a = emu.kernel_arg(0);
            let b = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, a, len as u32, false);
            emu.kernel_guard_access(rip, b, len as u32, false);
            let mut n = 0;
            while n < len && emu.maps.read_byte(a + n) == emu.maps.read_byte(b + n) {
                n += 1;
            }
            emu.set_kernel_ret(n);
        }
        "RtlEqualMemory" => {
            let a = emu.kernel_arg(0);
            let b = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let eq = (0..len).all(|i| emu.maps.read_byte(a + i) == emu.maps.read_byte(b + i));
            emu.set_kernel_ret(eq as u64);
        }

        // --- counted strings ------------------------------------------------------
        "RtlInitUnicodeString" | "RtlInitUnicodeStringEx" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            if src == 0 {
                write_unicode_header(emu, dst, 0, 0, 0);
            } else {
                let n = emu.maps.read_wide_string(src).encode_utf16().count() as u64 * 2;
                write_unicode_header(emu, dst, src, n, n + 2);
            }
            emu.set_kernel_ret(0);
        }
        "RtlInitAnsiString" | "RtlInitString" | "RtlInitAnsiStringEx" | "RtlInitUTF8String" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            if src == 0 {
                write_unicode_header(emu, dst, 0, 0, 0);
            } else {
                let n = emu.maps.read_string(src).len() as u64;
                write_unicode_header(emu, dst, src, n, n + 1);
            }
            emu.set_kernel_ret(0);
        }
        "RtlCopyUnicodeString" | "RtlCopyString" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let max = unicode_max(emu, dst);
            let dbuf = unicode_buf(emu, dst);
            if src == 0 {
                emu.maps.write_word(dst, 0);
            } else {
                let n = unicode_len(emu, src).min(max);
                let sbuf = unicode_buf(emu, src);
                if n > 0 && dbuf != 0 && sbuf != 0 {
                    crt::checked_copy(emu, dbuf, sbuf, n);
                }
                emu.maps.write_word(dst, n as u16);
                let cw = if symbol == "RtlCopyString" { 1 } else { 2 };
                if n + cw <= max && dbuf != 0 {
                    if cw == 2 {
                        emu.maps.write_word(dbuf + n, 0);
                    } else {
                        emu.maps.write_byte(dbuf + n, 0);
                    }
                }
            }
            emu.set_kernel_ret(0);
        }
        "RtlAppendUnicodeStringToString" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let n = unicode_len(emu, src) as usize;
            let sbuf = unicode_buf(emu, src);
            let bytes: Vec<u8> = (0..n)
                .map(|i| emu.maps.read_byte(sbuf + i as u64).unwrap_or(0))
                .collect();
            let st = unicode_append(emu, dst, &bytes);
            emu.set_kernel_ret(st);
        }
        "RtlAppendUnicodeToString" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let s = if src == 0 {
                String::new()
            } else {
                emu.maps.read_wide_string(src)
            };
            let st = unicode_append(emu, dst, &utf16_bytes(&s));
            emu.set_kernel_ret(st);
        }
        "RtlAppendStringToString" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let s = read_ansi_string(emu, src);
            let st = unicode_append(emu, dst, s.as_bytes());
            emu.set_kernel_ret(st);
        }
        "RtlCompareUnicodeString" | "RtlCompareString" => {
            let ci = emu.kernel_arg(2) != 0;
            let (a, b) = if symbol == "RtlCompareString" {
                (
                    read_ansi_string(emu, emu.kernel_arg(0)),
                    read_ansi_string(emu, emu.kernel_arg(1)),
                )
            } else {
                (
                    read_unicode_string(emu, emu.kernel_arg(0)),
                    read_unicode_string(emu, emu.kernel_arg(1)),
                )
            };
            emu.set_kernel_ret(compare_strings(&a, &b, ci) as u64);
        }
        "RtlEqualUnicodeString" | "RtlEqualString" => {
            let ci = emu.kernel_arg(2) != 0;
            let (a, b) = if symbol == "RtlEqualString" {
                (
                    read_ansi_string(emu, emu.kernel_arg(0)),
                    read_ansi_string(emu, emu.kernel_arg(1)),
                )
            } else {
                (
                    read_unicode_string(emu, emu.kernel_arg(0)),
                    read_unicode_string(emu, emu.kernel_arg(1)),
                )
            };
            emu.set_kernel_ret((compare_strings(&a, &b, ci) == 0) as u64);
        }
        "RtlPrefixUnicodeString" | "RtlPrefixString" => {
            let ci = emu.kernel_arg(2) != 0;
            let (p, s) = if symbol == "RtlPrefixString" {
                (
                    read_ansi_string(emu, emu.kernel_arg(0)),
                    read_ansi_string(emu, emu.kernel_arg(1)),
                )
            } else {
                (
                    read_unicode_string(emu, emu.kernel_arg(0)),
                    read_unicode_string(emu, emu.kernel_arg(1)),
                )
            };
            let ok = if ci {
                s.to_lowercase().starts_with(&p.to_lowercase())
            } else {
                s.starts_with(&p)
            };
            emu.set_kernel_ret(ok as u64);
        }
        "RtlUpcaseUnicodeString" | "RtlDowncaseUnicodeString" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let alloc = emu.kernel_arg(2) != 0;
            let s = read_unicode_string(emu, src);
            let s = if symbol == "RtlUpcaseUnicodeString" {
                s.to_uppercase()
            } else {
                s.to_lowercase()
            };
            if alloc {
                let (buf, len) = alloc_wide(emu, &s, symbol);
                write_unicode_header(emu, dst, buf, len, len + 2);
            } else {
                let bytes = utf16_bytes(&s);
                emu.maps.write_word(dst, 0);
                unicode_append(emu, dst, &bytes);
            }
            emu.set_kernel_ret(0);
        }
        "RtlUpcaseUnicodeChar" | "RtlDowncaseUnicodeChar" => {
            let c = emu.kernel_arg(0) as u16;
            let ch = char::from_u32(c as u32).unwrap_or('\0');
            let r = if symbol == "RtlUpcaseUnicodeChar" {
                ch.to_uppercase().next().unwrap_or(ch)
            } else {
                ch.to_lowercase().next().unwrap_or(ch)
            };
            emu.set_kernel_ret(r as u64);
        }
        "RtlAnsiCharToUnicodeChar" => {
            let pp = emu.kernel_arg(0);
            let p = emu.maps.read_qword(pp).unwrap_or(0);
            let c = emu.maps.read_byte(p).unwrap_or(0);
            emu.maps.write_qword(pp, p + 1);
            emu.set_kernel_ret(c as u64);
        }
        // RtlAnsiStringToUnicodeString(Dest, Source, AllocateDestinationString)
        "RtlAnsiStringToUnicodeString" | "RtlUTF8StringToUnicodeString" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let alloc = emu.kernel_arg(2) != 0;
            let s = read_ansi_string(emu, src);
            if alloc {
                let (buf, len) = alloc_wide(emu, &s, symbol);
                write_unicode_header(emu, dst, buf, len, len + 2);
                emu.set_kernel_ret(if buf != 0 { 0 } else { STATUS_NO_MEMORY });
            } else {
                emu.maps.write_word(dst, 0);
                let st = unicode_append(emu, dst, &utf16_bytes(&s));
                emu.set_kernel_ret(st);
            }
        }
        "RtlUnicodeStringToAnsiString" | "RtlUnicodeStringToUTF8String" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let alloc = emu.kernel_arg(2) != 0;
            let s = read_unicode_string(emu, src);
            if alloc {
                let buf = emu.kernel_alloc(
                    Region::Slab,
                    s.len() as u64 + 1,
                    "ansi-string",
                    symbol,
                    true,
                );
                if buf != 0 {
                    emu.maps.write_string(buf, &s);
                }
                write_unicode_header(emu, dst, buf, s.len() as u64, s.len() as u64 + 1);
                emu.set_kernel_ret(if buf != 0 { 0 } else { STATUS_NO_MEMORY });
            } else {
                emu.maps.write_word(dst, 0);
                let st = unicode_append(emu, dst, s.as_bytes());
                emu.set_kernel_ret(st);
            }
        }
        "RtlFreeUnicodeString" | "RtlFreeAnsiString" | "RtlFreeUTF8String" | "RtlFreeOemString" => {
            let us = emu.kernel_arg(0);
            let buf = unicode_buf(emu, us);
            if buf != 0 {
                emu.kernel_free(buf, symbol);
                write_unicode_header(emu, us, 0, 0, 0);
            }
            emu.set_kernel_ret(0);
        }
        "RtlUnicodeStringToInteger" | "RtlCharToInteger" => {
            let base = emu.kernel_arg(1) as u32;
            let out = emu.kernel_arg(2);
            let s = if symbol == "RtlCharToInteger" {
                emu.maps.read_string(emu.kernel_arg(0))
            } else {
                read_unicode_string(emu, emu.kernel_arg(0))
            };
            let t = s.trim();
            let (digits, radix) = if base == 0 {
                if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
                    (h, 16)
                } else if let Some(o) = t.strip_prefix("0o") {
                    (o, 8)
                } else if let Some(b) = t.strip_prefix("0b") {
                    (b, 2)
                } else {
                    (t, 10)
                }
            } else {
                (t, base)
            };
            let end = digits
                .find(|c: char| !c.is_digit(radix))
                .unwrap_or(digits.len());
            let v = u32::from_str_radix(&digits[..end], radix).unwrap_or(0);
            write_out_dword(emu, out, v);
            emu.set_kernel_ret(0);
        }
        "RtlIntegerToUnicodeString" | "RtlInt64ToUnicodeString" => {
            let v = emu.kernel_arg(0);
            let base = emu.kernel_arg(1) as u32;
            let dst = emu.kernel_arg(2);
            let v = if symbol == "RtlIntegerToUnicodeString" {
                v as u32 as u64
            } else {
                v
            };
            let s = match base {
                16 => format!("{:x}", v),
                8 => format!("{:o}", v),
                2 => format!("{:b}", v),
                _ => v.to_string(),
            };
            emu.maps.write_word(dst, 0);
            let st = unicode_append(emu, dst, &utf16_bytes(&s));
            emu.set_kernel_ret(st);
        }
        "RtlUnicodeToMultiByteN"
        | "RtlUpcaseUnicodeToMultiByteN"
        | "RtlUnicodeToOemN"
        | "RtlUnicodeToUTF8N" => {
            // (MultiByteString, MaxBytes, *BytesInMultiByte, UnicodeString, BytesInUnicode)
            let dst = emu.kernel_arg(0);
            let max = emu.kernel_arg(1);
            let out = emu.kernel_arg(2);
            let src = emu.kernel_arg(3);
            let n = (emu.kernel_arg(4) as u32 as u64) / 2;
            let s = emu.maps.read_wide_string_n(src, n as usize);
            let s = if symbol.contains("Upcase") {
                s.to_uppercase()
            } else {
                s
            };
            let bytes = s.as_bytes();
            let n = (bytes.len() as u64).min(max);
            emu.maps.write_bytes(dst, &bytes[..n as usize]);
            write_out_dword(emu, out, n as u32);
            emu.set_kernel_ret(0);
        }
        "RtlMultiByteToUnicodeN" | "RtlOemToUnicodeN" | "RtlUTF8ToUnicodeN" => {
            // (UnicodeString, MaxBytes, *BytesInUnicode, MultiByteString, BytesInMultiByte)
            let dst = emu.kernel_arg(0);
            let max = emu.kernel_arg(1);
            let out = emu.kernel_arg(2);
            let src = emu.kernel_arg(3);
            let n = emu.kernel_arg(4) as u32 as u64;
            let bytes: Vec<u8> = (0..n)
                .map(|i| emu.maps.read_byte(src + i).unwrap_or(0))
                .collect();
            let s = String::from_utf8_lossy(&bytes).to_string();
            let wide = utf16_bytes(&s);
            let n = (wide.len() as u64).min(max);
            emu.maps.write_bytes(dst, &wide[..n as usize]);
            write_out_dword(emu, out, n as u32);
            emu.set_kernel_ret(0);
        }
        "RtlUnicodeToMultiByteSize"
        | "RtlxUnicodeStringToAnsiSize"
        | "RtlMultiByteToUnicodeSize"
        | "RtlxAnsiStringToUnicodeSize" => {
            let v = match symbol {
                "RtlUnicodeToMultiByteSize" => {
                    let out = emu.kernel_arg(0);
                    let n = emu.kernel_arg(2) / 2;
                    write_out_dword(emu, out, n as u32);
                    0
                }
                "RtlMultiByteToUnicodeSize" => {
                    let out = emu.kernel_arg(0);
                    let n = emu.kernel_arg(2) * 2;
                    write_out_dword(emu, out, n as u32);
                    0
                }
                "RtlxUnicodeStringToAnsiSize" => unicode_len(emu, emu.kernel_arg(0)) / 2 + 1,
                _ => unicode_len(emu, emu.kernel_arg(0)) * 2 + 2,
            };
            emu.set_kernel_ret(v);
        }
        "RtlHashUnicodeString" => {
            let s = read_unicode_string(emu, emu.kernel_arg(0));
            let ci = emu.kernel_arg(1) != 0;
            let out = emu.kernel_arg(3);
            let s = if ci { s.to_uppercase() } else { s };
            let mut h: u32 = 0;
            for c in s.encode_utf16() {
                h = h.wrapping_mul(65599).wrapping_add(c as u32);
            }
            write_out_dword(emu, out, h);
            emu.set_kernel_ret(0);
        }
        "RtlStringFromGUID" => {
            let guid = emu.kernel_arg(0);
            let dst = emu.kernel_arg(1);
            let d1 = emu.maps.read_dword(guid).unwrap_or(0);
            let d2 = emu.maps.read_word(guid + 4).unwrap_or(0);
            let d3 = emu.maps.read_word(guid + 6).unwrap_or(0);
            let b: Vec<u8> = (0..8)
                .map(|i| emu.maps.read_byte(guid + 8 + i).unwrap_or(0))
                .collect();
            let s = format!(
                "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
                d1, d2, d3, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]
            );
            let (buf, len) = alloc_wide(emu, &s, symbol);
            write_unicode_header(emu, dst, buf, len, len + 2);
            emu.set_kernel_ret(0);
        }
        "RtlGUIDFromString" => {
            let s = read_unicode_string(emu, emu.kernel_arg(0));
            let out = emu.kernel_arg(1);
            let hex: Vec<u8> = s
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .collect::<String>()
                .as_bytes()
                .chunks(2)
                .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap_or("0"), 16).unwrap_or(0))
                .collect();
            if hex.len() == 16 {
                emu.maps
                    .write_dword(out, u32::from_be_bytes([hex[0], hex[1], hex[2], hex[3]]));
                emu.maps
                    .write_word(out + 4, u16::from_be_bytes([hex[4], hex[5]]));
                emu.maps
                    .write_word(out + 6, u16::from_be_bytes([hex[6], hex[7]]));
                emu.maps.write_bytes(out + 8, &hex[8..16]);
                emu.set_kernel_ret(0);
            } else {
                emu.set_kernel_ret(STATUS_INVALID_PARAMETER);
            }
        }

        // --- version ------------------------------------------------------------------
        "RtlGetVersion" => {
            let info = emu.kernel_arg(0);
            fill_version_info(emu, info);
            emu.set_kernel_ret(0);
        }
        "RtlVerifyVersionInfo" => emu.set_kernel_ret(0),
        "RtlIsNtDdiVersionAvailable" | "RtlIsServicePackVersionInstalled" => emu.set_kernel_ret(1),
        "RtlGetNtVersionNumbers" => {
            let (a, b, c) = (emu.kernel_arg(0), emu.kernel_arg(1), emu.kernel_arg(2));
            write_out_dword(emu, a, OS_MAJOR);
            write_out_dword(emu, b, OS_MINOR);
            write_out_dword(emu, c, NT_BUILD_NUMBER as u32 | 0xF000_0000);
            emu.set_kernel_ret(0);
        }
        "RtlGetNtProductType" => {
            let out = emu.kernel_arg(0);
            write_out_dword(emu, out, 1);
            emu.set_kernel_ret(1);
        }

        // --- registry helpers ---------------------------------------------------------
        // Nothing is stored under the emulated registry, so queries report the
        // value as absent and the driver falls back to its defaults.
        "RtlQueryRegistryValues" | "RtlQueryRegistryValuesEx" => {
            let path = emu.kernel_arg(1);
            let name = if path != 0 {
                emu.maps.read_wide_string(path)
            } else {
                String::new()
            };
            log::info!("{}: '{}'", symbol, name);
            emu.set_kernel_ret(STATUS_OBJECT_NAME_NOT_FOUND);
        }
        "RtlWriteRegistryValue"
        | "RtlCreateRegistryKey"
        | "RtlDeleteRegistryValue"
        | "RtlCheckRegistryKey" => {
            let path = emu.kernel_arg(1);
            let name = if path != 0 {
                emu.maps.read_wide_string(path)
            } else {
                String::new()
            };
            log::info!("{}: '{}'", symbol, name);
            emu.set_kernel_ret(0);
        }
        "RtlFormatCurrentUserKeyPath" => {
            let dst = emu.kernel_arg(0);
            let (buf, len) = alloc_wide(emu, "\\Registry\\User\\.DEFAULT", symbol);
            write_unicode_header(emu, dst, buf, len, len + 2);
            emu.set_kernel_ret(0);
        }

        // --- security descriptors -----------------------------------------------------
        "RtlCreateSecurityDescriptor" => {
            let sd = emu.kernel_arg(0);
            let rev = emu.kernel_arg(1) as u8;
            emu.maps.write_bytes(sd, &[0u8; SD_ABSOLUTE_SIZE as usize]);
            emu.maps.write_byte(sd + SD_REVISION, rev);
            emu.set_kernel_ret(0);
        }
        "RtlSetDaclSecurityDescriptor" | "RtlSetSaclSecurityDescriptor" => {
            let sd = emu.kernel_arg(0);
            let present = emu.kernel_arg(1) != 0;
            let acl = emu.kernel_arg(2);
            let defaulted = emu.kernel_arg(3) != 0;
            let (off, present_bit, def_bit) = if symbol == "RtlSetDaclSecurityDescriptor" {
                (SD_DACL, SE_DACL_PRESENT, SE_DACL_DEFAULTED)
            } else {
                (SD_SACL, SE_SACL_PRESENT, 0x20)
            };
            let mut ctl =
                emu.maps.read_word(sd + SD_CONTROL).unwrap_or(0) & !(present_bit | def_bit);
            if present {
                ctl |= present_bit;
                emu.maps.write_qword(sd + off, acl);
                if defaulted {
                    ctl |= def_bit;
                }
            } else {
                emu.maps.write_qword(sd + off, 0);
            }
            emu.maps.write_word(sd + SD_CONTROL, ctl);
            emu.set_kernel_ret(0);
        }
        "RtlSetOwnerSecurityDescriptor" | "RtlSetGroupSecurityDescriptor" => {
            let sd = emu.kernel_arg(0);
            let sid = emu.kernel_arg(1);
            let off = if symbol == "RtlSetOwnerSecurityDescriptor" {
                SD_OWNER
            } else {
                SD_GROUP
            };
            emu.maps.write_qword(sd + off, sid);
            emu.set_kernel_ret(0);
        }
        "RtlGetDaclSecurityDescriptor" | "RtlGetSaclSecurityDescriptor" => {
            // (SD, *Present, *Acl, *Defaulted)
            let sd = emu.kernel_arg(0);
            let present_out = emu.kernel_arg(1);
            let acl_out = emu.kernel_arg(2);
            let def_out = emu.kernel_arg(3);
            let (off, bit) = if symbol == "RtlGetDaclSecurityDescriptor" {
                (SD_DACL, SE_DACL_PRESENT)
            } else {
                (SD_SACL, SE_SACL_PRESENT)
            };
            let ctl = emu.maps.read_word(sd + SD_CONTROL).unwrap_or(0);
            let present = ctl & bit != 0;
            let acl = if present {
                emu.maps.read_qword(sd + off).unwrap_or(0)
            } else {
                0
            };
            if present_out != 0 {
                emu.maps.write_byte(present_out, present as u8);
            }
            write_out_qword(emu, acl_out, acl);
            if def_out != 0 {
                emu.maps.write_byte(def_out, 0);
            }
            emu.set_kernel_ret(0);
        }
        "RtlGetOwnerSecurityDescriptor" | "RtlGetGroupSecurityDescriptor" => {
            let sd = emu.kernel_arg(0);
            let sid_out = emu.kernel_arg(1);
            let def_out = emu.kernel_arg(2);
            let off = if symbol == "RtlGetOwnerSecurityDescriptor" {
                SD_OWNER
            } else {
                SD_GROUP
            };
            let sid = emu.maps.read_qword(sd + off).unwrap_or(0);
            write_out_qword(emu, sid_out, sid);
            if def_out != 0 {
                emu.maps.write_byte(def_out, 0);
            }
            emu.set_kernel_ret(0);
        }
        "RtlLengthSecurityDescriptor" => {
            let sd = emu.kernel_arg(0);
            let n = sd_self_relative_size(emu, sd);
            emu.set_kernel_ret(n);
        }
        "RtlValidSecurityDescriptor"
        | "RtlValidRelativeSecurityDescriptor"
        | "RtlValidSid"
        | "RtlValidAcl" => emu.set_kernel_ret(1),
        // RtlAbsoluteToSelfRelativeSD(Absolute, SelfRelative, *BufferLength)
        "RtlAbsoluteToSelfRelativeSD" | "RtlMakeSelfRelativeSD" => {
            let abs = emu.kernel_arg(0);
            let rel = emu.kernel_arg(1);
            let len_ptr = emu.kernel_arg(2);
            let need = sd_self_relative_size(emu, abs);
            let have = emu.maps.read_dword(len_ptr).unwrap_or(0) as u64;
            if have < need || rel == 0 {
                write_out_dword(emu, len_ptr, need as u32);
                emu.set_kernel_ret(STATUS_BUFFER_TOO_SMALL);
                return true;
            }
            // Self-relative header is 0x14 bytes: Revision, Sbz1, Control,
            // then four u32 offsets (Owner, Group, Sacl, Dacl).
            emu.maps.write_bytes(rel, &vec![0u8; need as usize]);
            emu.maps
                .write_byte(rel, emu.maps.read_byte(abs).unwrap_or(1));
            let ctl = emu.maps.read_word(abs + SD_CONTROL).unwrap_or(0) | SE_SELF_RELATIVE;
            emu.maps.write_word(rel + 2, ctl);
            let mut cursor = 0x14u64;
            for (i, part) in [SD_OWNER, SD_GROUP, SD_SACL, SD_DACL].iter().enumerate() {
                let p = emu.maps.read_qword(abs + part).unwrap_or(0);
                if p == 0 || !emu.maps.is_mapped(p) {
                    continue;
                }
                let n = if i < 2 {
                    sid_length(emu, p)
                } else {
                    emu.maps.read_word(p + ACL_SIZE).unwrap_or(0) as u64
                };
                emu.maps.memcpy(rel + cursor, p, n as usize);
                emu.maps.write_dword(rel + 4 + i as u64 * 4, cursor as u32);
                cursor += n;
            }
            write_out_dword(emu, len_ptr, need as u32);
            emu.set_kernel_ret(0);
        }
        "RtlSelfRelativeToAbsoluteSD" | "RtlSelfRelativeToAbsoluteSD2" => {
            emu.set_kernel_ret(STATUS_NOT_SUPPORTED);
        }
        "RtlCreateAcl" => {
            let acl = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let rev = emu.kernel_arg(2) as u8;
            emu.maps
                .write_bytes(acl, &vec![0u8; len.min(0x10000) as usize]);
            emu.maps.write_byte(acl, rev);
            emu.maps.write_word(acl + ACL_SIZE, len as u16);
            emu.maps.write_word(acl + ACL_ACE_COUNT, 0);
            emu.set_kernel_ret(0);
        }
        // RtlAddAccessAllowedAce(Acl, AceRevision, AccessMask, Sid)
        "RtlAddAccessAllowedAce"
        | "RtlAddAccessAllowedAceEx"
        | "RtlAddAccessDeniedAce"
        | "RtlAddAccessDeniedAceEx"
        | "RtlAddMandatoryAce" => {
            let acl = emu.kernel_arg(0);
            let ex = symbol.ends_with("Ex") || symbol == "RtlAddMandatoryAce";
            let (mask, sid) = if ex {
                (emu.kernel_arg(3) as u32, emu.kernel_arg(4))
            } else {
                (emu.kernel_arg(2) as u32, emu.kernel_arg(3))
            };
            let ace_type = if symbol.contains("Denied") {
                1
            } else if symbol.contains("Mandatory") {
                0x11
            } else {
                0
            };
            let st = add_ace(emu, acl, ace_type, mask, sid);
            emu.set_kernel_ret(st);
        }
        "RtlGetAce" => {
            let acl = emu.kernel_arg(0);
            let idx = emu.kernel_arg(1);
            let out = emu.kernel_arg(2);
            let count = emu.maps.read_word(acl + ACL_ACE_COUNT).unwrap_or(0) as u64;
            if idx >= count {
                emu.set_kernel_ret(STATUS_INVALID_PARAMETER);
                return true;
            }
            let mut off = ACL_HEADER_SIZE;
            for _ in 0..idx {
                off += emu.maps.read_word(acl + off + 2).unwrap_or(0) as u64;
            }
            write_out_qword(emu, out, acl + off);
            emu.set_kernel_ret(0);
        }
        "RtlLengthSid" => {
            let sid = emu.kernel_arg(0);
            let n = sid_length(emu, sid);
            emu.set_kernel_ret(n);
        }
        "RtlLengthRequiredSid" => {
            let n = emu.kernel_arg(0);
            emu.set_kernel_ret(8 + 4 * n);
        }
        "RtlInitializeSid" | "RtlInitializeSidEx" => {
            let sid = emu.kernel_arg(0);
            let auth = emu.kernel_arg(1);
            let count = emu.kernel_arg(2) as u8;
            emu.maps.write_byte(sid, 1);
            emu.maps.write_byte(sid + 1, count);
            if auth != 0 {
                emu.maps.memcpy(sid + 2, auth, 6);
            }
            emu.set_kernel_ret(0);
        }
        "RtlSubAuthoritySid" => {
            let sid = emu.kernel_arg(0);
            let i = emu.kernel_arg(1);
            emu.set_kernel_ret(sid + 8 + 4 * i);
        }
        "RtlSubAuthorityCountSid" => {
            let sid = emu.kernel_arg(0);
            emu.set_kernel_ret(sid + 1);
        }
        "RtlCopySid" => {
            let len = emu.kernel_arg(0);
            let dst = emu.kernel_arg(1);
            let src = emu.kernel_arg(2);
            let n = sid_length(emu, src);
            if n > len {
                emu.set_kernel_ret(STATUS_BUFFER_TOO_SMALL);
            } else {
                emu.maps.memcpy(dst, src, n as usize);
                emu.set_kernel_ret(0);
            }
        }
        "RtlEqualSid" | "RtlEqualPrefixSid" => {
            let a = emu.kernel_arg(0);
            let b = emu.kernel_arg(1);
            let n = sid_length(emu, a);
            let eq = n == sid_length(emu, b)
                && (0..n).all(|i| emu.maps.read_byte(a + i) == emu.maps.read_byte(b + i));
            emu.set_kernel_ret(eq as u64);
        }
        "RtlConvertSidToUnicodeString" => {
            let dst = emu.kernel_arg(0);
            let sid = emu.kernel_arg(1);
            let alloc = emu.kernel_arg(2) != 0;
            let count = emu.maps.read_byte(sid + 1).unwrap_or(0) as u64;
            let auth = emu.maps.read_byte(sid + 7).unwrap_or(0);
            let mut s = format!("S-1-{}", auth);
            for i in 0..count {
                s.push_str(&format!(
                    "-{}",
                    emu.maps.read_dword(sid + 8 + 4 * i).unwrap_or(0)
                ));
            }
            if alloc {
                let (buf, len) = alloc_wide(emu, &s, symbol);
                write_unicode_header(emu, dst, buf, len, len + 2);
            } else {
                emu.maps.write_word(dst, 0);
                unicode_append(emu, dst, &utf16_bytes(&s));
            }
            emu.set_kernel_ret(0);
        }

        // --- image helpers -------------------------------------------------------------
        "RtlImageNtHeader" | "RtlImageNtHeaderEx" => {
            let base = if symbol == "RtlImageNtHeader" {
                emu.kernel_arg(0)
            } else {
                emu.kernel_arg(1)
            };
            let out = emu.kernel_arg(3);
            let nt = if emu.maps.read_word(base) == Some(0x5a4d) {
                base + emu.maps.read_dword(base + 0x3c).unwrap_or(0) as u64
            } else {
                0
            };
            if symbol == "RtlImageNtHeader" {
                emu.set_kernel_ret(nt);
            } else {
                write_out_qword(emu, out, nt);
                emu.set_kernel_ret(if nt != 0 { 0 } else { 0xC000007B });
            }
        }
        // RtlImageDirectoryEntryToData(Base, MappedAsImage, Directory, *Size)
        "RtlImageDirectoryEntryToData" => {
            let base = emu.kernel_arg(0);
            let dir = emu.kernel_arg(2);
            let size_out = emu.kernel_arg(3);
            let mut result = 0;
            if emu.maps.read_word(base) == Some(0x5a4d) {
                let nt = base + emu.maps.read_dword(base + 0x3c).unwrap_or(0) as u64;
                let magic = emu.maps.read_word(nt + 0x18).unwrap_or(0);
                let dirs = if magic == 0x20b {
                    nt + 0x18 + 0x70
                } else {
                    nt + 0x18 + 0x60
                };
                let rva = emu.maps.read_dword(dirs + dir * 8).unwrap_or(0) as u64;
                let size = emu.maps.read_dword(dirs + dir * 8 + 4).unwrap_or(0);
                if rva != 0 {
                    result = base + rva;
                    write_out_dword(emu, size_out, size);
                }
            }
            emu.set_kernel_ret(result);
        }
        "RtlLookupFunctionEntry" | "RtlLookupFunctionTable" | "RtlPcToFileHeader" => {
            if symbol == "RtlPcToFileHeader" {
                let out = emu.kernel_arg(1);
                let base = emu.kernel.as_ref().map(|k| k.module.base).unwrap_or(0);
                write_out_qword(emu, out, base);
                emu.set_kernel_ret(base);
            } else {
                emu.set_kernel_ret(0);
            }
        }
        "RtlVirtualUnwind"
        | "RtlWalkFrameChain"
        | "RtlCaptureStackBackTrace"
        | "RtlCaptureContext"
        | "RtlRestoreContext"
        | "RtlAddFunctionTable"
        | "RtlDeleteFunctionTable"
        | "RtlInstallFunctionTableCallback"
        | "RtlUnwind" => {
            emu.set_kernel_ret(0);
        }
        "RtlUnwindEx" | "RtlRaiseException" | "RtlRaiseStatus" => {
            let what = if symbol == "RtlRaiseStatus" {
                format!("status 0x{:x}", emu.kernel_arg(0))
            } else {
                "exception record".to_string()
            };
            log::error!(
                "{}: {} — no SEH dispatch in kernel mode, stopping",
                symbol,
                what
            );
            emu.kernel_log_line(format!("{} ({})", symbol, what));
            emu.stop();
        }
        "RtlAssert" => {
            let msg = emu.maps.read_string(emu.kernel_arg(0));
            let file = emu.maps.read_string(emu.kernel_arg(1));
            let line = emu.kernel_arg(2);
            emu.kernel_log_line(format!("ASSERT {} at {}:{}", msg, file, line));
            emu.set_kernel_ret(0);
        }

        // --- bitmaps --------------------------------------------------------------------
        "RtlInitializeBitMap" => {
            let bm = emu.kernel_arg(0);
            let buf = emu.kernel_arg(1);
            let bits = emu.kernel_arg(2) as u32;
            emu.maps.write_dword(bm, bits);
            emu.maps.write_qword(bm + 8, buf);
            emu.set_kernel_ret(0);
        }
        "RtlClearAllBits" | "RtlSetAllBits" => {
            let bm = emu.kernel_arg(0);
            let bits = emu.maps.read_dword(bm).unwrap_or(0) as u64;
            let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
            let v = if symbol == "RtlSetAllBits" { 0xff } else { 0 };
            emu.maps
                .write_bytes(buf, &vec![v; bits.div_ceil(8) as usize]);
            emu.set_kernel_ret(0);
        }
        "RtlSetBit" | "RtlClearBit" | "RtlTestBit" | "RtlCheckBit" => {
            let bm = emu.kernel_arg(0);
            let bit = emu.kernel_arg(1);
            let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
            let byte = buf + bit / 8;
            let mut v = emu.maps.read_byte(byte).unwrap_or(0);
            let mask = 1u8 << (bit % 8);
            match symbol {
                "RtlSetBit" => {
                    v |= mask;
                    emu.maps.write_byte(byte, v);
                    emu.set_kernel_ret(0);
                }
                "RtlClearBit" => {
                    v &= !mask;
                    emu.maps.write_byte(byte, v);
                    emu.set_kernel_ret(0);
                }
                _ => emu.set_kernel_ret((v & mask != 0) as u64),
            }
        }
        "RtlSetBits" | "RtlClearBits" => {
            let bm = emu.kernel_arg(0);
            let start = emu.kernel_arg(1);
            let n = emu.kernel_arg(2);
            let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
            for bit in start..start + n {
                let byte = buf + bit / 8;
                let mut v = emu.maps.read_byte(byte).unwrap_or(0);
                if symbol == "RtlSetBits" {
                    v |= 1 << (bit % 8);
                } else {
                    v &= !(1 << (bit % 8));
                }
                emu.maps.write_byte(byte, v);
            }
            emu.set_kernel_ret(0);
        }
        "RtlFindClearBits"
        | "RtlFindClearBitsAndSet"
        | "RtlFindSetBits"
        | "RtlFindSetBitsAndClear"
        | "RtlFindNextForwardRunClear"
        | "RtlFindFirstRunClear"
        | "RtlFindLongestRunClear" => {
            let bm = emu.kernel_arg(0);
            let n = emu.kernel_arg(1).max(1);
            let want_set = symbol.contains("SetBits");
            let bits = emu.maps.read_dword(bm).unwrap_or(0) as u64;
            let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
            let get = |emu: &Emu, bit: u64| {
                emu.maps.read_byte(buf + bit / 8).unwrap_or(0) & (1 << (bit % 8)) != 0
            };
            let mut found = 0xffff_ffffu64;
            let mut run = 0;
            for bit in 0..bits {
                if get(emu, bit) == want_set {
                    run += 1;
                    if run == n {
                        found = bit + 1 - n;
                        break;
                    }
                } else {
                    run = 0;
                }
            }
            if found != 0xffff_ffff && (symbol.ends_with("AndSet") || symbol.ends_with("AndClear"))
            {
                for bit in found..found + n {
                    let byte = buf + bit / 8;
                    let mut v = emu.maps.read_byte(byte).unwrap_or(0);
                    if symbol.ends_with("AndSet") {
                        v |= 1 << (bit % 8);
                    } else {
                        v &= !(1 << (bit % 8));
                    }
                    emu.maps.write_byte(byte, v);
                }
            }
            emu.set_kernel_ret(found);
        }
        "RtlNumberOfSetBits" | "RtlNumberOfClearBits" => {
            let bm = emu.kernel_arg(0);
            let bits = emu.maps.read_dword(bm).unwrap_or(0) as u64;
            let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
            let set = (0..bits)
                .filter(|bit| {
                    emu.maps.read_byte(buf + bit / 8).unwrap_or(0) & (1 << (bit % 8)) != 0
                })
                .count() as u64;
            emu.set_kernel_ret(if symbol == "RtlNumberOfSetBits" {
                set
            } else {
                bits - set
            });
        }

        // --- time, random, misc -----------------------------------------------------------
        "RtlTimeToTimeFields" => {
            let t = emu.maps.read_qword(emu.kernel_arg(0)).unwrap_or(0);
            let out = emu.kernel_arg(1);
            // Only the fields a log line needs: seconds since 1601 split up.
            let secs = t / 10_000_000;
            let days = secs / 86400;
            let (y, m, d) = civil_from_days(days as i64 - 134774); // 1601 -> 1970 offset
            emu.maps.write_word(out, y as u16);
            emu.maps.write_word(out + 2, m as u16);
            emu.maps.write_word(out + 4, d as u16);
            emu.maps.write_word(out + 6, ((secs / 3600) % 24) as u16);
            emu.maps.write_word(out + 8, ((secs / 60) % 60) as u16);
            emu.maps.write_word(out + 10, (secs % 60) as u16);
            emu.maps.write_word(out + 12, ((t / 10_000) % 1000) as u16);
            emu.maps.write_word(out + 14, ((days + 1) % 7) as u16);
            emu.set_kernel_ret(0);
        }
        "RtlTimeFieldsToTime" => {
            let out = emu.kernel_arg(1);
            emu.maps.write_qword(out, system_time_now());
            emu.set_kernel_ret(1);
        }
        "RtlRandom" | "RtlRandomEx" => {
            let seed_ptr = emu.kernel_arg(0);
            let seed = emu.maps.read_dword(seed_ptr).unwrap_or(1);
            let next = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
            emu.maps.write_dword(seed_ptr, next);
            emu.set_kernel_ret((next >> 1) as u64);
        }
        "RtlQueryFeatureConfiguration"
        | "RtlQueryFeatureConfigurationChangeStamp"
        | "RtlRegisterFeatureConfigurationChangeNotification"
        | "RtlUnregisterFeatureConfigurationChangeNotification"
        | "RtlNotifyFeatureUsage" => {
            emu.set_kernel_ret(if symbol == "RtlQueryFeatureConfiguration" {
                STATUS_NOT_FOUND
            } else {
                0
            });
        }
        "RtlIsGenericTableEmptyAvl" | "RtlIsGenericTableEmpty" => emu.set_kernel_ret(1),
        "RtlInitializeGenericTableAvl" | "RtlInitializeGenericTable" => {
            let t = emu.kernel_arg(0);
            emu.maps.write_bytes(t, &[0u8; 0x40]);
            emu.set_kernel_ret(0);
        }
        // RtlInsertElementGenericTableAvl(Table, Buffer, BufferSize, *NewElement)
        "RtlInsertElementGenericTableAvl" | "RtlInsertElementGenericTable" => {
            let buf = emu.kernel_arg(1);
            let size = emu.kernel_arg(2);
            let new_out = emu.kernel_arg(3);
            let e = emu.kernel_alloc(Region::Slab, size.max(1), "generic-table", symbol, true);
            if e != 0 {
                emu.maps.memcpy(e, buf, size as usize);
            }
            if new_out != 0 {
                emu.maps.write_byte(new_out, 1);
            }
            emu.set_kernel_ret(e);
        }
        "RtlLookupElementGenericTableAvl"
        | "RtlLookupElementGenericTable"
        | "RtlEnumerateGenericTableAvl"
        | "RtlEnumerateGenericTable"
        | "RtlEnumerateGenericTableWithoutSplayingAvl"
        | "RtlGetElementGenericTableAvl"
        | "RtlLookupFirstMatchingElementGenericTableAvl" => emu.set_kernel_ret(0),
        "RtlDeleteElementGenericTableAvl" | "RtlDeleteElementGenericTable" => emu.set_kernel_ret(1),
        "RtlNumberGenericTableElementsAvl" | "RtlNumberGenericTableElements" => {
            emu.set_kernel_ret(0)
        }
        "RtlIsNameLegalDOS8Dot3" => emu.set_kernel_ret(1),
        "RtlSetBitMapEx" | "RtlCopyLuid" => emu.set_kernel_ret(0),
        _ => return false,
    }
    true
}

/// Howard Hinnant's days-to-civil, for `RtlTimeToTimeFields`.
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub const SURFACE: &[&str] = &[
    "RtlCopyMemory",
    "RtlMoveMemory",
    "RtlZeroMemory",
    "RtlSecureZeroMemory",
    "RtlFillMemory",
    "RtlCompareMemory",
    "RtlEqualMemory",
    "RtlInitUnicodeString",
    "RtlInitUnicodeStringEx",
    "RtlInitAnsiString",
    "RtlInitString",
    "RtlInitUTF8String",
    "RtlCopyUnicodeString",
    "RtlCopyString",
    "RtlAppendUnicodeStringToString",
    "RtlAppendUnicodeToString",
    "RtlAppendStringToString",
    "RtlCompareUnicodeString",
    "RtlCompareString",
    "RtlEqualUnicodeString",
    "RtlEqualString",
    "RtlPrefixUnicodeString",
    "RtlPrefixString",
    "RtlUpcaseUnicodeString",
    "RtlDowncaseUnicodeString",
    "RtlUpcaseUnicodeChar",
    "RtlDowncaseUnicodeChar",
    "RtlAnsiCharToUnicodeChar",
    "RtlAnsiStringToUnicodeString",
    "RtlUnicodeStringToAnsiString",
    "RtlUTF8StringToUnicodeString",
    "RtlUnicodeStringToUTF8String",
    "RtlFreeUnicodeString",
    "RtlFreeAnsiString",
    "RtlFreeUTF8String",
    "RtlUnicodeStringToInteger",
    "RtlCharToInteger",
    "RtlIntegerToUnicodeString",
    "RtlInt64ToUnicodeString",
    "RtlUnicodeToMultiByteN",
    "RtlUpcaseUnicodeToMultiByteN",
    "RtlMultiByteToUnicodeN",
    "RtlUnicodeToMultiByteSize",
    "RtlMultiByteToUnicodeSize",
    "RtlxUnicodeStringToAnsiSize",
    "RtlxAnsiStringToUnicodeSize",
    "RtlHashUnicodeString",
    "RtlStringFromGUID",
    "RtlGUIDFromString",
    "RtlGetVersion",
    "RtlVerifyVersionInfo",
    "RtlIsNtDdiVersionAvailable",
    "RtlGetNtVersionNumbers",
    "RtlGetNtProductType",
    "RtlQueryRegistryValues",
    "RtlQueryRegistryValuesEx",
    "RtlWriteRegistryValue",
    "RtlCreateRegistryKey",
    "RtlDeleteRegistryValue",
    "RtlCheckRegistryKey",
    "RtlFormatCurrentUserKeyPath",
    "RtlCreateSecurityDescriptor",
    "RtlSetDaclSecurityDescriptor",
    "RtlSetSaclSecurityDescriptor",
    "RtlSetOwnerSecurityDescriptor",
    "RtlSetGroupSecurityDescriptor",
    "RtlGetDaclSecurityDescriptor",
    "RtlGetSaclSecurityDescriptor",
    "RtlGetOwnerSecurityDescriptor",
    "RtlGetGroupSecurityDescriptor",
    "RtlLengthSecurityDescriptor",
    "RtlValidSecurityDescriptor",
    "RtlValidRelativeSecurityDescriptor",
    "RtlAbsoluteToSelfRelativeSD",
    "RtlMakeSelfRelativeSD",
    "RtlCreateAcl",
    "RtlAddAccessAllowedAce",
    "RtlAddAccessAllowedAceEx",
    "RtlAddAccessDeniedAce",
    "RtlAddMandatoryAce",
    "RtlGetAce",
    "RtlValidAcl",
    "RtlLengthSid",
    "RtlLengthRequiredSid",
    "RtlInitializeSid",
    "RtlSubAuthoritySid",
    "RtlSubAuthorityCountSid",
    "RtlCopySid",
    "RtlEqualSid",
    "RtlValidSid",
    "RtlConvertSidToUnicodeString",
    "RtlImageNtHeader",
    "RtlImageNtHeaderEx",
    "RtlImageDirectoryEntryToData",
    "RtlLookupFunctionEntry",
    "RtlPcToFileHeader",
    "RtlVirtualUnwind",
    "RtlWalkFrameChain",
    "RtlCaptureStackBackTrace",
    "RtlCaptureContext",
    "RtlUnwind",
    "RtlUnwindEx",
    "RtlRaiseException",
    "RtlRaiseStatus",
    "RtlAssert",
    "RtlInitializeBitMap",
    "RtlClearAllBits",
    "RtlSetAllBits",
    "RtlSetBit",
    "RtlClearBit",
    "RtlTestBit",
    "RtlSetBits",
    "RtlClearBits",
    "RtlFindClearBits",
    "RtlFindClearBitsAndSet",
    "RtlFindSetBits",
    "RtlFindSetBitsAndClear",
    "RtlFindNextForwardRunClear",
    "RtlNumberOfSetBits",
    "RtlNumberOfClearBits",
    "RtlTimeToTimeFields",
    "RtlTimeFieldsToTime",
    "RtlRandom",
    "RtlRandomEx",
    "RtlQueryFeatureConfiguration",
    "RtlNotifyFeatureUsage",
    "RtlIsGenericTableEmptyAvl",
    "RtlInitializeGenericTableAvl",
    "RtlInsertElementGenericTableAvl",
    "RtlLookupElementGenericTableAvl",
    "RtlDeleteElementGenericTableAvl",
    "RtlEnumerateGenericTableAvl",
    "RtlNumberGenericTableElementsAvl",
    "RtlIsNameLegalDOS8Dot3",
];
