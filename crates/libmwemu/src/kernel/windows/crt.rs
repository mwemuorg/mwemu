//! The C runtime a driver imports from ntoskrnl: `memcpy`, `strlen`, the
//! wide-character family, and the `printf` family (narrow, wide and
//! `va_list` forms) that also backs `DbgPrint`.
//!
//! Copies and fills go through the memory guard: a `memcpy` into a freed pool
//! chunk is a use-after-free that no instruction-level check would see,
//! because the copy happens inside the kernel.

use super::*;

/// How much of `[addr, addr + len)` is backed by mapped memory, stopping at
/// the first unmapped page. A copy with a caller-controlled length runs off
/// the end of its buffer on a real kernel too; here the guard has already
/// reported the overrun, and the rest of the copy would only be a very slow
/// way of writing nothing.
pub fn mapped_extent(emu: &Emu, addr: u64, len: u64) -> u64 {
    if !emu.maps.is_mapped(addr) {
        return 0;
    }
    let mut end = addr;
    let limit = addr.saturating_add(len);
    while end < limit {
        let next_page = (end | 0xfff) + 1;
        if !emu.maps.is_mapped(end) {
            return end - addr;
        }
        end = next_page.min(limit);
    }
    len
}

/// Copy `len` bytes, reporting either end that lands in a freed or
/// out-of-bounds chunk.
pub fn checked_copy(emu: &mut Emu, dst: u64, src: u64, len: u64) {
    if len == 0 {
        return;
    }
    let rip = emu.pc();
    emu.kernel_guard_range(rip, src, len, false);
    emu.kernel_guard_range(rip, dst, len, true);
    emu.kernel_guard_access(rip, src, len as u32, false);
    emu.kernel_guard_access(rip, dst, len as u32, true);
    let n = mapped_extent(emu, src, len).min(mapped_extent(emu, dst, len));
    if n < len {
        log::warn!(
            "copy of {} bytes from 0x{:x} to 0x{:x} runs off mapped memory after {} bytes",
            len,
            src,
            dst,
            n
        );
    }
    let mut buf = vec![0u8; n as usize];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
    }
    emu.maps.write_bytes(dst, &buf);
}

pub fn checked_fill(emu: &mut Emu, dst: u64, byte: u8, len: u64) {
    if len == 0 {
        return;
    }
    let rip = emu.pc();
    emu.kernel_guard_range(rip, dst, len, true);
    emu.kernel_guard_access(rip, dst, len as u32, true);
    let n = mapped_extent(emu, dst, len);
    if n < len {
        log::warn!(
            "fill of {} bytes at 0x{:x} runs off mapped memory after {} bytes",
            len,
            dst,
            n
        );
    }
    emu.maps.write_bytes(dst, &vec![byte; n as usize]);
}

fn read_wide(emu: &Emu, p: u64) -> String {
    if p == 0 {
        "(null)".to_string()
    } else {
        emu.maps.read_wide_string(p)
    }
}

fn read_narrow(emu: &Emu, p: u64) -> String {
    if p == 0 {
        "(null)".to_string()
    } else {
        emu.maps.read_string(p)
    }
}

/// Where a variadic formatter takes its arguments from.
pub enum ArgSource {
    /// Argument index into the current call (register/stack per the ABI).
    Call(usize),
    /// A `va_list`: on x64 a pointer to qwords on the caller's stack.
    VaList(u64),
}

impl ArgSource {
    fn next(&mut self, emu: &Emu) -> u64 {
        match self {
            ArgSource::Call(idx) => {
                let v = emu.kernel_arg(*idx);
                *idx += 1;
                v
            }
            ArgSource::VaList(p) => {
                let v = emu.maps.read_qword(*p).unwrap_or(0);
                *p += 8;
                v
            }
        }
    }
}

/// Expand a `printf`-style format. `wide` selects what a bare `%s` / `%c`
/// means (wide in `swprintf`, narrow in `sprintf`); the explicit `%ws`, `%S`,
/// `%hs`, `%ls`, `%wZ` (UNICODE_STRING) and `%Z` (ANSI_STRING) forms are
/// honoured in both. Width, precision and flags are applied where it matters
/// for a value (`%08x`, `%-20s`), which is what driver logs use.
pub fn format_args(emu: &mut Emu, fmt: &str, args: &mut ArgSource, wide: bool) -> String {
    let mut out = String::new();
    let mut chars = fmt.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        if chars.peek() == Some(&'%') {
            chars.next();
            out.push('%');
            continue;
        }
        let mut left = false;
        let mut zero = false;
        let mut alt = false;
        while let Some(f) = chars.peek().copied() {
            match f {
                '-' => left = true,
                '0' => zero = true,
                '#' => alt = true,
                '+' | ' ' => {}
                _ => break,
            }
            chars.next();
        }
        let mut width = 0usize;
        if chars.peek() == Some(&'*') {
            chars.next();
            width = args.next(emu) as usize;
        }
        while let Some(d) = chars.peek().and_then(|c| c.to_digit(10)) {
            width = width * 10 + d as usize;
            chars.next();
        }
        let mut precision: Option<usize> = None;
        if chars.peek() == Some(&'.') {
            chars.next();
            let mut p = 0usize;
            if chars.peek() == Some(&'*') {
                chars.next();
                p = args.next(emu) as usize;
            }
            while let Some(d) = chars.peek().and_then(|c| c.to_digit(10)) {
                p = p * 10 + d as usize;
                chars.next();
            }
            precision = Some(p);
        }
        // Length modifiers: h, hh, l, ll, I, I32, I64, z, j, t, q, w.
        let mut is_wide = wide;
        let mut is_64 = false;
        let mut is_short = false;
        loop {
            match chars.peek().copied() {
                Some('l') => {
                    chars.next();
                    if chars.peek() == Some(&'l') {
                        chars.next();
                        is_64 = true;
                    } else {
                        is_wide = true;
                    }
                }
                Some('h') => {
                    chars.next();
                    is_wide = false;
                    is_short = true;
                }
                Some('w') => {
                    chars.next();
                    is_wide = true;
                }
                Some('I') => {
                    chars.next();
                    let mut digits = String::new();
                    while let Some(d) = chars.peek().filter(|c| c.is_ascii_digit()) {
                        digits.push(*d);
                        chars.next();
                    }
                    is_64 = digits != "32";
                }
                Some('z' | 'j' | 't' | 'q' | 'L') => {
                    chars.next();
                    is_64 = true;
                }
                _ => break,
            }
        }
        let Some(conv) = chars.next() else { break };
        let pad = |s: String, numeric: bool| -> String {
            if s.chars().count() >= width {
                return s;
            }
            let fill = width - s.chars().count();
            if left {
                format!("{}{}", s, " ".repeat(fill))
            } else if zero && numeric {
                let (sign, rest) = if let Some(r) = s.strip_prefix('-') {
                    ("-", r.to_string())
                } else {
                    ("", s)
                };
                format!("{}{}{}", sign, "0".repeat(fill), rest)
            } else {
                format!("{}{}", " ".repeat(fill), s)
            }
        };
        let trunc = |s: String| -> String {
            match precision {
                Some(p) => s.chars().take(p).collect(),
                None => s,
            }
        };
        let piece = match conv {
            'd' | 'i' => {
                let v = args.next(emu);
                let v = if is_64 {
                    v as i64
                } else if is_short {
                    v as i16 as i64
                } else {
                    v as i32 as i64
                };
                pad(v.to_string(), true)
            }
            'u' => {
                let v = args.next(emu);
                let v = if is_64 {
                    v
                } else if is_short {
                    v as u16 as u64
                } else {
                    v as u32 as u64
                };
                pad(v.to_string(), true)
            }
            'x' | 'X' | 'o' => {
                let v = args.next(emu);
                let v = if is_64 {
                    v
                } else if is_short {
                    v as u16 as u64
                } else {
                    v as u32 as u64
                };
                let s = match conv {
                    'x' => format!("{:x}", v),
                    'X' => format!("{:X}", v),
                    _ => format!("{:o}", v),
                };
                let s = if alt && v != 0 {
                    format!("0{}{}", if conv == 'o' { "" } else { "x" }, s)
                } else {
                    s
                };
                pad(s, true)
            }
            'p' => pad(format!("{:016X}", args.next(emu)), false),
            'c' | 'C' => {
                let v = args.next(emu);
                let w = (conv == 'C') != is_wide;
                let ch = if w {
                    char::from_u32(v as u16 as u32)
                } else {
                    Some(v as u8 as char)
                };
                pad(ch.unwrap_or('?').to_string(), false)
            }
            's' | 'S' => {
                let p = args.next(emu);
                // In a wide format `%S` is narrow; in a narrow one it is wide.
                let w = (conv == 'S') != is_wide;
                let s = if w {
                    read_wide(emu, p)
                } else {
                    read_narrow(emu, p)
                };
                pad(trunc(s), false)
            }
            'Z' => {
                let p = args.next(emu);
                let s = if is_wide {
                    read_unicode_string(emu, p)
                } else {
                    read_ansi_string(emu, p)
                };
                pad(trunc(s), false)
            }
            'f' | 'g' | 'e' | 'F' | 'G' | 'E' => {
                // Floats travel in XMM registers on x64; not modelled.
                args.next(emu);
                "0.0".to_string()
            }
            'n' => {
                let p = args.next(emu);
                if p != 0 {
                    emu.maps.write_dword(p, out.len() as u32);
                }
                String::new()
            }
            other => format!("%{}", other),
        };
        out.push_str(&piece);
    }
    out
}

/// `DbgPrint`-style: the format string is narrow and lives at `fmt_addr`;
/// arguments start at call argument `first_arg`.
pub fn format_narrow(emu: &mut Emu, fmt_addr: u64, first_arg: usize) -> String {
    let fmt = emu.maps.read_string(fmt_addr);
    format_args(emu, &fmt, &mut ArgSource::Call(first_arg), false)
}

/// Write `s` as a narrow string into `buf` of `count` bytes, C-truncated.
/// Returns the number of characters written, or -1 when truncated (the
/// `_snprintf` contract).
fn write_narrow_result(emu: &mut Emu, buf: u64, count: u64, s: &str) -> u64 {
    let bytes = s.as_bytes();
    if buf == 0 || count == 0 {
        return bytes.len() as u64;
    }
    if (bytes.len() as u64) < count {
        checked_copy_bytes(emu, buf, bytes);
        emu.maps.write_byte(buf + bytes.len() as u64, 0);
        bytes.len() as u64
    } else {
        checked_copy_bytes(emu, buf, &bytes[..count as usize]);
        u64::MAX
    }
}

fn write_wide_result(emu: &mut Emu, buf: u64, count: u64, s: &str) -> u64 {
    let units: Vec<u16> = s.encode_utf16().collect();
    if buf == 0 || count == 0 {
        return units.len() as u64;
    }
    let bytes: Vec<u8> = units.iter().flat_map(|u| u.to_le_bytes()).collect();
    if (units.len() as u64) < count {
        checked_copy_bytes(emu, buf, &bytes);
        emu.maps.write_word(buf + bytes.len() as u64, 0);
        units.len() as u64
    } else {
        checked_copy_bytes(emu, buf, &bytes[..(count * 2) as usize]);
        u64::MAX
    }
}

fn checked_copy_bytes(emu: &mut Emu, dst: u64, bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    let rip = emu.pc();
    emu.kernel_guard_range(rip, dst, bytes.len() as u64, true);
    emu.kernel_guard_access(rip, dst, bytes.len() as u32, true);
    emu.maps.write_bytes(dst, bytes);
}

fn wide_len(emu: &Emu, p: u64) -> u64 {
    let mut n = 0;
    while emu.maps.read_word(p + n * 2).unwrap_or(0) != 0 {
        n += 1;
    }
    n
}

fn to_lower_ascii(s: &str) -> String {
    s.to_lowercase()
}

fn cmp_result(o: std::cmp::Ordering) -> u64 {
    match o {
        std::cmp::Ordering::Less => (-1i64) as u64,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- logging ------------------------------------------------------------
        "DbgPrint" | "DbgPrintReturnControlC" => {
            let fmt = emu.kernel_arg(0);
            let line = format_narrow(emu, fmt, 1);
            emu.kernel_log_line(line);
            emu.set_kernel_ret(0);
        }
        "DbgPrintEx" => {
            let fmt = emu.kernel_arg(2);
            let line = format_narrow(emu, fmt, 3);
            emu.kernel_log_line(line);
            emu.set_kernel_ret(0);
        }
        "vDbgPrintEx" => {
            let fmt = emu.maps.read_string(emu.kernel_arg(2));
            let va = emu.kernel_arg(3);
            let line = format_args(emu, &fmt, &mut ArgSource::VaList(va), false);
            emu.kernel_log_line(line);
            emu.set_kernel_ret(0);
        }
        "vDbgPrintExWithPrefix" => {
            let prefix = emu.maps.read_string(emu.kernel_arg(0));
            let fmt = emu.maps.read_string(emu.kernel_arg(3));
            let va = emu.kernel_arg(4);
            let line = format_args(emu, &fmt, &mut ArgSource::VaList(va), false);
            emu.kernel_log_line(format!("{}{}", prefix, line));
            emu.set_kernel_ret(0);
        }
        "DbgBreakPoint"
        | "DbgBreakPointWithStatus"
        | "DbgSetDebugFilterState"
        | "DbgQueryDebugFilterState" => {
            emu.kernel_log_line(format!("{} called", symbol));
            emu.set_kernel_ret(0);
        }

        // --- memory ---------------------------------------------------------------
        "memcpy" | "memmove" | "memcpy_s" | "memmove_s" | "RtlCopyBytes" | "RtlMoveBytes" => {
            let dst = emu.kernel_arg(0);
            let (src, len) = if symbol.ends_with("_s") {
                (emu.kernel_arg(2), emu.kernel_arg(3))
            } else {
                (emu.kernel_arg(1), emu.kernel_arg(2))
            };
            checked_copy(emu, dst, src, len);
            emu.set_kernel_ret(if symbol.ends_with("_s") { 0 } else { dst });
        }
        "memset" | "RtlFillBytes" | "RtlFillMemoryUlong" | "RtlFillMemoryUlonglong" => {
            let dst = emu.kernel_arg(0);
            match symbol {
                "memset" => {
                    let byte = emu.kernel_arg(1) as u8;
                    let len = emu.kernel_arg(2);
                    checked_fill(emu, dst, byte, len);
                }
                "RtlFillBytes" => {
                    let len = emu.kernel_arg(1);
                    let byte = emu.kernel_arg(2) as u8;
                    checked_fill(emu, dst, byte, len);
                }
                _ => {
                    let len = emu.kernel_arg(1);
                    let pat = emu.kernel_arg(2);
                    let unit = if symbol.ends_with("Ulong") { 4 } else { 8 };
                    let rip = emu.pc();
                    emu.kernel_guard_range(rip, dst, len, true);
                    let len = mapped_extent(emu, dst, len);
                    let mut i = 0;
                    while i + unit <= len {
                        if unit == 4 {
                            emu.maps.write_dword(dst + i, pat as u32);
                        } else {
                            emu.maps.write_qword(dst + i, pat);
                        }
                        i += unit;
                    }
                }
            }
            emu.set_kernel_ret(dst);
        }
        "memcmp" | "RtlCompareBytes" => {
            let a = emu.kernel_arg(0);
            let b = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, a, len as u32, false);
            emu.kernel_guard_access(rip, b, len as u32, false);
            let mut r = 0u64;
            for i in 0..len {
                let x = emu.maps.read_byte(a + i).unwrap_or(0);
                let y = emu.maps.read_byte(b + i).unwrap_or(0);
                if x != y {
                    r = cmp_result(x.cmp(&y));
                    break;
                }
            }
            emu.set_kernel_ret(r);
        }
        "memchr" => {
            let p = emu.kernel_arg(0);
            let c = emu.kernel_arg(1) as u8;
            let len = emu.kernel_arg(2);
            let found = (0..len)
                .find(|i| emu.maps.read_byte(p + i) == Some(c))
                .map(|i| p + i)
                .unwrap_or(0);
            emu.set_kernel_ret(found);
        }

        // --- narrow strings ---------------------------------------------------------
        "strlen" => {
            let s = emu.maps.read_string(emu.kernel_arg(0));
            emu.set_kernel_ret(s.len() as u64);
        }
        "strnlen" | "strnlen_s" => {
            let s = emu.maps.read_string(emu.kernel_arg(0));
            let max = emu.kernel_arg(1);
            emu.set_kernel_ret((s.len() as u64).min(max));
        }
        "strcmp" | "strncmp" | "_stricmp" | "_strnicmp" | "_strcmpi" | "stricmp" | "strnicmp" => {
            let mut a = emu.maps.read_string(emu.kernel_arg(0));
            let mut b = emu.maps.read_string(emu.kernel_arg(1));
            if symbol.starts_with("strn") || symbol.starts_with("_strn") {
                let n = emu.kernel_arg(2) as usize;
                a.truncate(n.min(a.len()));
                b.truncate(n.min(b.len()));
            }
            if symbol.contains("icmp") || symbol.contains("cmpi") {
                a = to_lower_ascii(&a);
                b = to_lower_ascii(&b);
            }
            emu.set_kernel_ret(cmp_result(a.as_bytes().cmp(b.as_bytes())));
        }
        "strcpy" | "strcpy_s" => {
            let dst = emu.kernel_arg(0);
            let src = if symbol == "strcpy_s" {
                emu.kernel_arg(2)
            } else {
                emu.kernel_arg(1)
            };
            let s = emu.maps.read_string(src);
            checked_copy(emu, dst, src, s.len() as u64 + 1);
            emu.set_kernel_ret(if symbol == "strcpy_s" { 0 } else { dst });
        }
        "strncpy" | "strncpy_s" => {
            let dst = emu.kernel_arg(0);
            let (src, n) = if symbol == "strncpy_s" {
                (emu.kernel_arg(2), emu.kernel_arg(3).min(emu.kernel_arg(1)))
            } else {
                (emu.kernel_arg(1), emu.kernel_arg(2))
            };
            let s = emu.maps.read_string(src);
            let copy = (s.len() as u64).min(n);
            checked_copy(emu, dst, src, copy);
            if copy < n {
                checked_fill(
                    emu,
                    dst + copy,
                    0,
                    (n - copy).min(1 + (symbol == "strncpy") as u64 * (n - copy)),
                );
            }
            emu.set_kernel_ret(if symbol == "strncpy_s" { 0 } else { dst });
        }
        "strcat" | "strcat_s" | "strncat" | "strncat_s" => {
            let dst = emu.kernel_arg(0);
            let s_arg = if symbol.ends_with("_s") { 2 } else { 1 };
            let src = emu.kernel_arg(s_arg);
            let mut s = emu.maps.read_string(src);
            if symbol.starts_with("strn") {
                let n = emu.kernel_arg(s_arg + 1) as usize;
                s.truncate(n.min(s.len()));
            }
            let dlen = emu.maps.read_string(dst).len() as u64;
            let mut bytes = s.into_bytes();
            bytes.push(0);
            checked_copy_bytes(emu, dst + dlen, &bytes);
            emu.set_kernel_ret(if symbol.ends_with("_s") { 0 } else { dst });
        }
        "strchr" | "strrchr" => {
            let p = emu.kernel_arg(0);
            let c = emu.kernel_arg(1) as u8;
            let s = emu.maps.read_string(p);
            let pos = if symbol == "strchr" {
                s.as_bytes().iter().position(|b| *b == c)
            } else {
                s.as_bytes().iter().rposition(|b| *b == c)
            };
            let r = match pos {
                Some(i) => p + i as u64,
                None if c == 0 => p + s.len() as u64,
                None => 0,
            };
            emu.set_kernel_ret(r);
        }
        "strstr" => {
            let p = emu.kernel_arg(0);
            let s = emu.maps.read_string(p);
            let needle = emu.maps.read_string(emu.kernel_arg(1));
            let r = s.find(&needle).map(|i| p + i as u64).unwrap_or(0);
            emu.set_kernel_ret(r);
        }
        "_strlwr" | "_strupr" | "strlwr" | "strupr" => {
            let p = emu.kernel_arg(0);
            let s = emu.maps.read_string(p);
            let t = if symbol.contains("lwr") {
                s.to_lowercase()
            } else {
                s.to_uppercase()
            };
            emu.maps.write_string(p, &t);
            emu.set_kernel_ret(p);
        }
        "toupper" | "tolower" | "_toupper" | "_tolower" => {
            let c = emu.kernel_arg(0) as u8 as char;
            let r = if symbol.contains("upper") {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            };
            emu.set_kernel_ret(r as u64);
        }
        "isdigit" | "isspace" | "isalpha" | "isalnum" | "isupper" | "islower" | "isxdigit"
        | "isprint" => {
            let c = emu.kernel_arg(0) as u8 as char;
            let r = match symbol {
                "isdigit" => c.is_ascii_digit(),
                "isspace" => c.is_ascii_whitespace(),
                "isalpha" => c.is_ascii_alphabetic(),
                "isalnum" => c.is_ascii_alphanumeric(),
                "isupper" => c.is_ascii_uppercase(),
                "islower" => c.is_ascii_lowercase(),
                "isxdigit" => c.is_ascii_hexdigit(),
                _ => c.is_ascii_graphic() || c == ' ',
            };
            emu.set_kernel_ret(r as u64);
        }
        "atoi" | "atol" | "_atoi64" | "strtol" | "strtoul" | "_strtoui64" | "_strtoi64"
        | "strtoll" | "strtoull" => {
            let s = emu.maps.read_string(emu.kernel_arg(0));
            let (end_ptr, base) = if symbol.starts_with("atoi")
                || symbol.starts_with("atol")
                || symbol == "_atoi64"
            {
                (0, 10)
            } else {
                (emu.kernel_arg(1), emu.kernel_arg(2) as u32)
            };
            let (v, used) = parse_int(&s, base);
            if end_ptr != 0 {
                emu.maps
                    .write_qword(end_ptr, emu.kernel_arg(0) + used as u64);
            }
            emu.set_kernel_ret(v as u64);
        }
        "_itoa" | "_ltoa" | "_ultoa" | "_i64toa" | "_ui64toa" | "_itoa_s" | "_ultoa_s" => {
            let v = emu.kernel_arg(0);
            let (buf, radix) = if symbol.ends_with("_s") {
                (emu.kernel_arg(1), emu.kernel_arg(3) as u32)
            } else {
                (emu.kernel_arg(1), emu.kernel_arg(2) as u32)
            };
            let s = match radix {
                16 => format!("{:x}", v),
                8 => format!("{:o}", v),
                2 => format!("{:b}", v),
                _ => {
                    if symbol.contains("u") {
                        v.to_string()
                    } else {
                        (v as i64).to_string()
                    }
                }
            };
            emu.maps.write_string(buf, &s);
            emu.set_kernel_ret(if symbol.ends_with("_s") { 0 } else { buf });
        }

        // --- wide strings ---------------------------------------------------------------
        "wcslen" => {
            let n = wide_len(emu, emu.kernel_arg(0));
            emu.set_kernel_ret(n);
        }
        "wcsnlen" | "wcsnlen_s" => {
            let n = wide_len(emu, emu.kernel_arg(0));
            let max = emu.kernel_arg(1);
            emu.set_kernel_ret(n.min(max));
        }
        "wcscmp" | "wcsncmp" | "_wcsicmp" | "_wcsnicmp" | "wcsicmp" | "wcsnicmp" => {
            let mut a = emu.maps.read_wide_string(emu.kernel_arg(0));
            let mut b = emu.maps.read_wide_string(emu.kernel_arg(1));
            if symbol.contains("wcsn") {
                let n = emu.kernel_arg(2) as usize;
                a = a.chars().take(n).collect();
                b = b.chars().take(n).collect();
            }
            if symbol.contains("icmp") {
                a = a.to_lowercase();
                b = b.to_lowercase();
            }
            let au: Vec<u16> = a.encode_utf16().collect();
            let bu: Vec<u16> = b.encode_utf16().collect();
            emu.set_kernel_ret(cmp_result(au.cmp(&bu)));
        }
        "wcscpy" | "wcscpy_s" => {
            let dst = emu.kernel_arg(0);
            let src = if symbol == "wcscpy_s" {
                emu.kernel_arg(2)
            } else {
                emu.kernel_arg(1)
            };
            let n = wide_len(emu, src);
            checked_copy(emu, dst, src, (n + 1) * 2);
            emu.set_kernel_ret(if symbol == "wcscpy_s" { 0 } else { dst });
        }
        "wcsncpy" | "wcsncpy_s" => {
            let dst = emu.kernel_arg(0);
            let (src, n) = if symbol == "wcsncpy_s" {
                (emu.kernel_arg(2), emu.kernel_arg(3).min(emu.kernel_arg(1)))
            } else {
                (emu.kernel_arg(1), emu.kernel_arg(2))
            };
            let len = wide_len(emu, src).min(n);
            checked_copy(emu, dst, src, len * 2);
            if len < n {
                emu.maps.write_word(dst + len * 2, 0);
            }
            emu.set_kernel_ret(if symbol == "wcsncpy_s" { 0 } else { dst });
        }
        "wcscat" | "wcscat_s" | "wcsncat" | "wcsncat_s" => {
            let dst = emu.kernel_arg(0);
            let s_arg = if symbol.ends_with("_s") { 2 } else { 1 };
            let src = emu.kernel_arg(s_arg);
            let mut s = emu.maps.read_wide_string(src);
            if symbol.starts_with("wcsn") {
                let n = emu.kernel_arg(s_arg + 1) as usize;
                s = s.chars().take(n).collect();
            }
            let dlen = wide_len(emu, dst);
            let mut bytes: Vec<u8> = s.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
            bytes.extend_from_slice(&[0, 0]);
            checked_copy_bytes(emu, dst + dlen * 2, &bytes);
            emu.set_kernel_ret(if symbol.ends_with("_s") { 0 } else { dst });
        }
        "wcschr" | "wcsrchr" => {
            let p = emu.kernel_arg(0);
            let c = emu.kernel_arg(1) as u16;
            let n = wide_len(emu, p);
            let units: Vec<u16> = (0..n)
                .map(|i| emu.maps.read_word(p + i * 2).unwrap_or(0))
                .collect();
            let pos = if symbol == "wcschr" {
                units.iter().position(|u| *u == c)
            } else {
                units.iter().rposition(|u| *u == c)
            };
            let r = match pos {
                Some(i) => p + i as u64 * 2,
                None if c == 0 => p + n * 2,
                None => 0,
            };
            emu.set_kernel_ret(r);
        }
        "wcsstr" => {
            let p = emu.kernel_arg(0);
            let s = emu.maps.read_wide_string(p);
            let needle = emu.maps.read_wide_string(emu.kernel_arg(1));
            let r = s
                .find(&needle)
                .map(|i| p + s[..i].encode_utf16().count() as u64 * 2)
                .unwrap_or(0);
            emu.set_kernel_ret(r);
        }
        "_wcslwr" | "_wcsupr" | "wcslwr" | "wcsupr" => {
            let p = emu.kernel_arg(0);
            let s = emu.maps.read_wide_string(p);
            let t = if symbol.contains("lwr") {
                s.to_lowercase()
            } else {
                s.to_uppercase()
            };
            emu.maps.write_wide_string(p, &t);
            emu.set_kernel_ret(p);
        }
        "towupper" | "towlower" => {
            let c = char::from_u32(emu.kernel_arg(0) as u16 as u32).unwrap_or('\0');
            let r = if symbol == "towupper" {
                c.to_uppercase().next()
            } else {
                c.to_lowercase().next()
            };
            emu.set_kernel_ret(r.unwrap_or(c) as u64);
        }
        "iswdigit" | "iswspace" | "iswalpha" | "iswalnum" | "iswxdigit" => {
            let c = char::from_u32(emu.kernel_arg(0) as u16 as u32).unwrap_or('\0');
            let r = match symbol {
                "iswdigit" => c.is_ascii_digit(),
                "iswspace" => c.is_whitespace(),
                "iswalpha" => c.is_alphabetic(),
                "iswalnum" => c.is_alphanumeric(),
                _ => c.is_ascii_hexdigit(),
            };
            emu.set_kernel_ret(r as u64);
        }
        "_wtoi" | "_wtol" | "_wtoi64" | "wcstol" | "wcstoul" | "_wcstoui64" | "_wcstoi64"
        | "wcstoll" | "wcstoull" => {
            let s = emu.maps.read_wide_string(emu.kernel_arg(0));
            let (end_ptr, base) = if symbol.starts_with("_wto") {
                (0, 10)
            } else {
                (emu.kernel_arg(1), emu.kernel_arg(2) as u32)
            };
            let (v, used) = parse_int(&s, base);
            if end_ptr != 0 {
                emu.maps
                    .write_qword(end_ptr, emu.kernel_arg(0) + used as u64 * 2);
            }
            emu.set_kernel_ret(v as u64);
        }
        "mbstowcs" | "wcstombs" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let n = emu.kernel_arg(2) as usize;
            if symbol == "mbstowcs" {
                let s: String = emu.maps.read_string(src).chars().take(n).collect();
                if dst != 0 {
                    emu.maps.write_wide_string(dst, &s);
                }
                emu.set_kernel_ret(s.encode_utf16().count() as u64);
            } else {
                let s: String = emu.maps.read_wide_string(src).chars().take(n).collect();
                if dst != 0 {
                    emu.maps.write_string(dst, &s);
                }
                emu.set_kernel_ret(s.len() as u64);
            }
        }

        // --- printf family -----------------------------------------------------------------
        "sprintf" | "_snprintf" | "sprintf_s" | "_snprintf_s" => {
            let buf = emu.kernel_arg(0);
            let (count, fmt_idx) = match symbol {
                "sprintf" => (u64::MAX, 1),
                "_snprintf" => (emu.kernel_arg(1), 2),
                "sprintf_s" => (emu.kernel_arg(1), 2),
                _ => (emu.kernel_arg(2).min(emu.kernel_arg(1)), 3),
            };
            let fmt = emu.maps.read_string(emu.kernel_arg(fmt_idx));
            let s = format_args(emu, &fmt, &mut ArgSource::Call(fmt_idx + 1), false);
            let r = write_narrow_result(emu, buf, count, &s);
            emu.set_kernel_ret(r);
        }
        "vsprintf" | "_vsnprintf" | "vsprintf_s" | "_vsnprintf_s" => {
            let buf = emu.kernel_arg(0);
            let (count, fmt_idx) = match symbol {
                "vsprintf" => (u64::MAX, 1),
                "_vsnprintf" | "vsprintf_s" => (emu.kernel_arg(1), 2),
                _ => (emu.kernel_arg(2).min(emu.kernel_arg(1)), 3),
            };
            let fmt = emu.maps.read_string(emu.kernel_arg(fmt_idx));
            let va = emu.kernel_arg(fmt_idx + 1);
            let s = format_args(emu, &fmt, &mut ArgSource::VaList(va), false);
            let r = write_narrow_result(emu, buf, count, &s);
            emu.set_kernel_ret(r);
        }
        "swprintf" | "_snwprintf" | "swprintf_s" | "_snwprintf_s" => {
            let buf = emu.kernel_arg(0);
            let (count, fmt_idx) = match symbol {
                "swprintf" => (u64::MAX, 1),
                "_snwprintf" | "swprintf_s" => (emu.kernel_arg(1), 2),
                _ => (emu.kernel_arg(2).min(emu.kernel_arg(1)), 3),
            };
            let fmt = emu.maps.read_wide_string(emu.kernel_arg(fmt_idx));
            let s = format_args(emu, &fmt, &mut ArgSource::Call(fmt_idx + 1), true);
            let r = write_wide_result(emu, buf, count, &s);
            emu.set_kernel_ret(r);
        }
        "vswprintf" | "_vsnwprintf" | "vswprintf_s" | "_vsnwprintf_s" => {
            let buf = emu.kernel_arg(0);
            let (count, fmt_idx) = match symbol {
                "vswprintf" => (u64::MAX, 1),
                "_vsnwprintf" | "vswprintf_s" => (emu.kernel_arg(1), 2),
                _ => (emu.kernel_arg(2).min(emu.kernel_arg(1)), 3),
            };
            let fmt = emu.maps.read_wide_string(emu.kernel_arg(fmt_idx));
            let va = emu.kernel_arg(fmt_idx + 1);
            let s = format_args(emu, &fmt, &mut ArgSource::VaList(va), true);
            let r = write_wide_result(emu, buf, count, &s);
            emu.set_kernel_ret(r);
        }
        // ntstrsafe out-of-line forms: (Dest, DestSize, Format, ...)
        "RtlStringCbPrintfA"
        | "RtlStringCchPrintfA"
        | "RtlStringCbPrintfW"
        | "RtlStringCchPrintfW" => {
            let buf = emu.kernel_arg(0);
            let size = emu.kernel_arg(1);
            let wide = symbol.ends_with('W');
            let count = if symbol.contains("Cb") && wide {
                size / 2
            } else {
                size
            };
            let fmt = if wide {
                emu.maps.read_wide_string(emu.kernel_arg(2))
            } else {
                emu.maps.read_string(emu.kernel_arg(2))
            };
            let s = format_args(emu, &fmt, &mut ArgSource::Call(3), wide);
            let r = if wide {
                write_wide_result(emu, buf, count, &s)
            } else {
                write_narrow_result(emu, buf, count, &s)
            };
            emu.set_kernel_ret(if r == u64::MAX { 0x80000005 } else { 0 });
        }
        "RtlStringCbCopyA" | "RtlStringCchCopyA" | "RtlStringCbCopyW" | "RtlStringCchCopyW" => {
            let buf = emu.kernel_arg(0);
            let size = emu.kernel_arg(1);
            let wide = symbol.ends_with('W');
            let count = if symbol.contains("Cb") && wide {
                size / 2
            } else {
                size
            };
            let s = if wide {
                emu.maps.read_wide_string(emu.kernel_arg(2))
            } else {
                emu.maps.read_string(emu.kernel_arg(2))
            };
            let r = if wide {
                write_wide_result(emu, buf, count, &s)
            } else {
                write_narrow_result(emu, buf, count, &s)
            };
            emu.set_kernel_ret(if r == u64::MAX { 0x80000005 } else { 0 });
        }

        // --- runtime support ------------------------------------------------------------------
        // SEH personality: only reached through exception dispatch, which
        // kernel mode does not model. Continue the search.
        "__C_specific_handler" | "_C_specific_handler" => emu.set_kernel_ret(1),
        "__chkstk"
        | "_chkstk"
        | "__security_init_cookie"
        | "_local_unwind"
        | "__GSHandlerCheck" => {
            emu.set_kernel_ret(0);
        }
        "_purecall"
        | "__report_gsfailure"
        | "__report_rangecheckfailure"
        | "abort"
        | "_invoke_watson" => {
            log::error!("{}: the driver aborted", symbol);
            emu.kernel_log_line(format!("{} called", symbol));
            emu.stop();
        }
        "rand" => {
            let v = emu.pos.wrapping_mul(1_103_515_245).wrapping_add(12345);
            emu.set_kernel_ret((v >> 16) & 0x7fff);
        }
        "srand" | "qsort" | "bsearch" => emu.set_kernel_ret(0),
        "__cxa_atexit" | "_onexit" | "atexit" => emu.set_kernel_ret(0),
        _ => return false,
    }
    true
}

/// Parse a C-style integer with an optional sign and `0x` prefix. Returns the
/// value and the number of characters consumed.
fn parse_int(s: &str, base: u32) -> (i64, usize) {
    let t = s.trim_start();
    let skipped = s.len() - t.len();
    let (neg, t2) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let mut radix = base;
    let mut digits = t2;
    if (radix == 0 || radix == 16) && (digits.starts_with("0x") || digits.starts_with("0X")) {
        digits = &digits[2..];
        radix = 16;
    } else if radix == 0 && digits.starts_with('0') && digits.len() > 1 {
        radix = 8;
    } else if radix == 0 {
        radix = 10;
    }
    let end = digits
        .find(|c: char| !c.is_digit(radix))
        .unwrap_or(digits.len());
    let v = u64::from_str_radix(&digits[..end], radix).unwrap_or(0) as i64;
    let used = skipped + (t.len() - t2.len()) + (t2.len() - digits.len()) + end;
    (if neg { -v } else { v }, used)
}

pub const SURFACE: &[&str] = &[
    "DbgPrint",
    "DbgPrintEx",
    "vDbgPrintEx",
    "vDbgPrintExWithPrefix",
    "DbgBreakPoint",
    "DbgBreakPointWithStatus",
    "DbgSetDebugFilterState",
    "memcpy",
    "memmove",
    "memcpy_s",
    "memmove_s",
    "memset",
    "memcmp",
    "memchr",
    "RtlCopyBytes",
    "RtlFillBytes",
    "RtlCompareBytes",
    "RtlFillMemoryUlong",
    "RtlFillMemoryUlonglong",
    "strlen",
    "strnlen",
    "strcmp",
    "strncmp",
    "_stricmp",
    "_strnicmp",
    "_strcmpi",
    "strcpy",
    "strcpy_s",
    "strncpy",
    "strncpy_s",
    "strcat",
    "strcat_s",
    "strncat",
    "strncat_s",
    "strchr",
    "strrchr",
    "strstr",
    "_strlwr",
    "_strupr",
    "toupper",
    "tolower",
    "isdigit",
    "isspace",
    "isalpha",
    "isalnum",
    "isupper",
    "islower",
    "isxdigit",
    "isprint",
    "atoi",
    "atol",
    "_atoi64",
    "strtol",
    "strtoul",
    "_strtoui64",
    "_strtoi64",
    "_itoa",
    "_ultoa",
    "_i64toa",
    "_ui64toa",
    "wcslen",
    "wcsnlen",
    "wcscmp",
    "wcsncmp",
    "_wcsicmp",
    "_wcsnicmp",
    "wcscpy",
    "wcscpy_s",
    "wcsncpy",
    "wcsncpy_s",
    "wcscat",
    "wcscat_s",
    "wcsncat",
    "wcsncat_s",
    "wcschr",
    "wcsrchr",
    "wcsstr",
    "_wcslwr",
    "_wcsupr",
    "towupper",
    "towlower",
    "iswdigit",
    "iswspace",
    "iswalpha",
    "iswalnum",
    "iswxdigit",
    "_wtoi",
    "_wtol",
    "_wtoi64",
    "wcstol",
    "wcstoul",
    "_wcstoui64",
    "mbstowcs",
    "wcstombs",
    "sprintf",
    "_snprintf",
    "sprintf_s",
    "_snprintf_s",
    "vsprintf",
    "_vsnprintf",
    "vsprintf_s",
    "_vsnprintf_s",
    "swprintf",
    "_snwprintf",
    "swprintf_s",
    "_snwprintf_s",
    "vswprintf",
    "_vsnwprintf",
    "vswprintf_s",
    "_vsnwprintf_s",
    "RtlStringCbPrintfA",
    "RtlStringCchPrintfA",
    "RtlStringCbPrintfW",
    "RtlStringCchPrintfW",
    "RtlStringCbCopyA",
    "RtlStringCchCopyA",
    "RtlStringCbCopyW",
    "RtlStringCchCopyW",
    "__C_specific_handler",
    "__chkstk",
    "_purecall",
    "__report_gsfailure",
    "rand",
    "srand",
    "qsort",
];
