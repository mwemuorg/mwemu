//! The C runtime a driver imports from ntoskrnl: `memcpy`, `strlen`, the
//! wide-character family, and the `printf` family (narrow, wide and
//! `va_list` forms) that also backs `DbgPrint`.
//!
//! Copies and fills go through the memory guard: a `memcpy` into a freed pool
//! chunk is a use-after-free that no instruction-level check would see,
//! because the copy happens inside the kernel.

use crate::api::abi::CallFrame;
use crate::api::abi::windows::ArgumentError;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::memory::read_array;
use crate::api::windows::policy::api_try;
use crate::exception::types::ExceptionType;
use crate::kernel::guard::{ChunkOrigin, FindingKind};
use crate::kernel::{HandlerRegistry, kernel_handlers, return_one, return_zero};
use crate::maps::mem64::Permission;
use crate::windows::constants::crt::{_TRUNCATE, EINVAL, ERANGE, STRUNCATE};

use super::*;

/// How much of `[addr, addr + len)` is backed by mapped memory, stopping at
/// the first unmapped page. A copy with a caller-controlled length runs off
/// the end of its buffer on a real kernel too; here the guard has already
/// reported the overrun, and the rest of the copy would only be a very slow
/// way of writing nothing.
pub fn mapped_extent(emu: &Emu, address: u64, length: u64) -> u64 {
    let limit = address.saturating_add(length);
    let mut end = address;
    while end < limit {
        let Some(map) = emu.maps.get_mem_by_addr(end) else {
            break;
        };
        let next = map.get_bottom().min(limit);
        if next <= end {
            break;
        }
        end = next;
    }
    end - address
}

fn check_invalid_address(emu: &mut Emu, rip: u64, addr: u64, len: u64, is_write: bool) {
    const NULL_PAGE: u64 = 0x1000;
    const NEAR_MAX: u64 = 0xfffffffffffffff0;
    let invalid = !(NULL_PAGE..NEAR_MAX).contains(&addr) || !emu.maps.is_mapped(addr);
    if !invalid {
        return;
    }
    let dir = if is_write { "write to" } else { "read from" };
    log::warn!(
        "copy {} 0x{:x} ({} bytes): invalid address at 0x{:x}",
        dir,
        addr,
        len,
        rip
    );
    emu.kernel_report(
        FindingKind::InvalidAddress,
        rip,
        addr,
        len as u32,
        ChunkOrigin::default(),
    );
}

/// Copy `len` bytes, reporting either end that lands in a freed or
/// out-of-bounds chunk.
pub fn checked_copy(emu: &mut Emu, dst: u64, src: u64, len: u64) {
    if len == 0 {
        return;
    }
    let rip = emu.pc();
    check_invalid_address(emu, rip, src, len, false);
    check_invalid_address(emu, rip, dst, len, true);
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
    if !emu.maps.copy_range(dst, src, n as usize) {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
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
    if !emu.maps.fill_range(dst, byte, n as usize) {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
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
#[derive(Clone)]
pub enum ArgSource {
    /// Argument index into the current call (register/stack per the ABI).
    Call(usize),
    /// A `va_list`: on x64 a pointer to qwords on the caller's stack.
    VaList { base: u64, index: usize },
}

impl ArgSource {
    fn next(&mut self, emu: &Emu) -> Result<u64, ArgumentError> {
        let call = WinApi(Stdcall);
        Ok(match self {
            ArgSource::Call(idx) => {
                let v = call.arg(emu, *idx)?;
                *idx += 1;
                v
            }
            ArgSource::VaList { base, index } => {
                let frame = CallFrame::new(emu, call);
                let size = size_of::<u64>();
                let value = frame.memory_arg(*index, *base, index.checked_mul(size), size)?;
                *index += 1;
                value
            }
        })
    }
}

/// Expand a `printf`-style format. `wide` selects what a bare `%s` / `%c`
/// means (wide in `swprintf`, narrow in `sprintf`); the explicit `%ws`, `%S`,
/// `%hs`, `%ls`, `%wZ` (UNICODE_STRING) and `%Z` (ANSI_STRING) forms are
/// honoured in both. Width, precision and flags are applied where it matters
/// for a value (`%08x`, `%-20s`), which is what driver logs use.
pub fn format_args(
    emu: &mut Emu,
    fmt: &str,
    args: &mut ArgSource,
    wide: bool,
) -> Result<String, ArgumentError> {
    let mut checked = args.clone();
    let (text, has_counts) = render_args(emu, fmt, &mut checked, wide, false)?;
    if has_counts {
        // Validate the whole argument frame before %n writes. Render again so
        // those writes remain visible to later string conversions.
        render_args(emu, fmt, args, wide, true).map(|(text, _)| text)
    } else {
        *args = checked;
        Ok(text)
    }
}

fn render_args(
    emu: &mut Emu,
    fmt: &str,
    args: &mut ArgSource,
    wide: bool,
    write_counts: bool,
) -> Result<(String, bool), ArgumentError> {
    let mut out = String::new();
    let mut has_counts = false;
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
            width = args.next(emu)? as usize;
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
                p = args.next(emu)? as usize;
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
                let v = args.next(emu)?;
                let v = if is_64 {
                    v as i64
                } else if is_short {
                    i64::from(v as i16)
                } else {
                    i64::from(v as i32)
                };
                pad(v.to_string(), true)
            }
            'u' => {
                let v = args.next(emu)?;
                let v = if is_64 {
                    v
                } else if is_short {
                    u64::from(v as u16)
                } else {
                    u64::from(v as u32)
                };
                pad(v.to_string(), true)
            }
            'x' | 'X' | 'o' => {
                let v = args.next(emu)?;
                let v = if is_64 {
                    v
                } else if is_short {
                    u64::from(v as u16)
                } else {
                    u64::from(v as u32)
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
            'p' => pad(format!("{:016X}", args.next(emu)?), false),
            'c' | 'C' => {
                let v = args.next(emu)?;
                let w = (conv == 'C') != is_wide;
                let ch = if w {
                    char::from_u32(u32::from(v as u16))
                } else {
                    Some(v as u8 as char)
                };
                pad(ch.unwrap_or('?').to_string(), false)
            }
            's' | 'S' => {
                let p = args.next(emu)?;
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
                let p = args.next(emu)?;
                let s = if is_wide {
                    read_unicode_string(emu, p)
                } else {
                    read_ansi_string(emu, p)
                };
                pad(trunc(s), false)
            }
            'f' | 'g' | 'e' | 'F' | 'G' | 'E' => {
                // Floats travel in XMM registers on x64; not modelled.
                args.next(emu)?;
                "0.0".to_string()
            }
            'n' => {
                let p = args.next(emu)?;
                if p != 0 {
                    has_counts = true;
                    if write_counts {
                        emu.maps.write_dword(p, out.len() as u32);
                    }
                }
                String::new()
            }
            other => format!("%{}", other),
        };
        out.push_str(&piece);
    }
    Ok((out, has_counts))
}

/// `DbgPrint`-style: the format string is narrow and lives at `fmt_addr`;
/// arguments start at call argument `first_arg`.
pub fn format_narrow(
    emu: &mut Emu,
    fmt_addr: u64,
    first_arg: usize,
) -> Result<String, ArgumentError> {
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

fn cmp_result(o: std::cmp::Ordering) -> u64 {
    match o {
        std::cmp::Ordering::Less => (-1i64) as u64,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- logging ------------------------------------------------------------
        "DbgPrint" | "DbgPrintReturnControlC" => DbgPrint,
        "DbgPrintEx" => DbgPrintEx,
        "vDbgPrintEx" => vDbgPrintEx,
        "vDbgPrintExWithPrefix" => vDbgPrintExWithPrefix,
        "AslLogCallPrintf" => AslLogCallPrintf,

        "DbgBreakPoint"
        | "DbgBreakPointWithStatus"
        | "DbgSetDebugFilterState"
        | "DbgQueryDebugFilterState" => debug_stub,

        // --- memory ---------------------------------------------------------------
        "memcpy" | "memmove" | "RtlCopyBytes" | "RtlMoveBytes" => memcpy,
        "memcpy_s"|"memmove_s"=>secure_copy,
        "memset" | "RtlFillBytes" | "RtlFillMemoryUlong" | "RtlFillMemoryUlonglong" => memset,
        "memcmp" | "RtlCompareBytes" => memcmp,
        "memchr" => memchr,

        // --- narrow strings ---------------------------------------------------------
        "strlen" => strlen,
        "strnlen" | "strnlen_s" => strnlen,
        "strcmp" | "strncmp" | "_stricmp" | "_strnicmp" | "_strcmpi" | "stricmp" | "strnicmp" => strcmp,
        "strcpy" | "strcpy_s" => strcpy,
        "strncpy" | "strncpy_s" => strncpy,
        "strcat" | "strcat_s" | "strncat" | "strncat_s" => strcat,
        "strchr" | "strrchr" => strchr,
        "strstr" => strstr,
        "_strlwr" | "_strupr" | "strlwr" | "strupr" => _strlwr,
        "toupper" | "tolower" | "_toupper" | "_tolower" => toupper,
        "isdigit" | "isspace" | "isalpha" | "isalnum" | "isupper" | "islower" | "isxdigit"
        | "isprint" => isdigit,
        "atoi" | "atol" | "_atoi64" | "strtol" | "strtoul" | "_strtoui64" | "_strtoi64"
        | "strtoll" | "strtoull" => atoi,
        "_itoa" | "_ltoa" | "_ultoa" | "_i64toa" | "_ui64toa" | "_itoa_s" | "_ultoa_s" => _itoa,

        // --- wide strings ---------------------------------------------------------------
        "wcslen" => wcslen,
        "wcsnlen" | "wcsnlen_s" => wcsnlen,
        "wcscmp" | "wcsncmp" | "_wcsicmp" | "_wcsnicmp" | "wcsicmp" | "wcsnicmp" => wcscmp,
        "wcscpy" | "wcscpy_s" => wcscpy,
        "wcsncpy" | "wcsncpy_s" => wcsncpy,
        "wcscat" | "wcscat_s" | "wcsncat" | "wcsncat_s" => wcscat,
        "wcschr" | "wcsrchr" => wcschr,
        "wcsstr" => wcsstr,
        "_wcslwr" | "_wcsupr" | "wcslwr" | "wcsupr" => _wcslwr,
        "towupper" | "towlower" => towupper,
        "iswdigit" | "iswspace" | "iswalpha" | "iswalnum" | "iswxdigit" => iswdigit,
        "_wtoi" | "_wtol" | "_wtoi64" | "wcstol" | "wcstoul" | "_wcstoui64" | "_wcstoi64"
        | "wcstoll" | "wcstoull" => _wtoi,
        "mbstowcs" | "wcstombs" => mbstowcs,

        // --- printf family -----------------------------------------------------------------
        "sprintf" | "_snprintf" | "sprintf_s" | "_snprintf_s" => sprintf,
        "vsprintf" | "_vsnprintf" | "vsprintf_s" | "_vsnprintf_s" => vsprintf,
        "swprintf" | "_snwprintf" | "swprintf_s" | "_snwprintf_s" => swprintf,
        "vswprintf" | "_vsnwprintf" | "vswprintf_s" | "_vsnwprintf_s" => vswprintf,
        // ntstrsafe out-of-line forms: (Dest, DestSize, Format, ...)
        "RtlStringCbPrintfA"
        | "RtlStringCchPrintfA"
        | "RtlStringCbPrintfW"
        | "RtlStringCchPrintfW" => RtlStringCbPrintfA,
        "RtlStringCbCopyA" | "RtlStringCchCopyA" | "RtlStringCbCopyW" | "RtlStringCchCopyW" => RtlStringCbCopyA,

        // --- runtime support ------------------------------------------------------------------
        // SEH personality: only reached through exception dispatch, which
        // kernel mode does not model. Continue the search.
        "__C_specific_handler" | "_C_specific_handler" => return_one,
        "__chkstk" | "_chkstk" | "_local_unwind" | "__GSHandlerCheck" => return_zero,
        "__security_init_cookie" => return_zero,
        "__security_check_cookie" | "_security_check_cookie" => __security_check_cookie,
        "_purecall"
        | "__report_gsfailure"
        | "__report_rangecheckfailure"
        | "abort"
        | "_invoke_watson" => abort_call,
        "rand" => rand,
        "srand" | "qsort" | "bsearch" => return_zero,
        "__cxa_atexit" | "_onexit" | "atexit" => return_zero,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn DbgPrint(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let fmt = api_try!(emu, call.arg(emu, 0));
    let line = api_try!(emu, format_narrow(emu, fmt, 1));
    emu.kernel_log_line(line);
    call.set_return(emu, 0);
}

fn DbgPrintEx(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let fmt = api_try!(emu, call.arg(emu, 2));
    let line = api_try!(emu, format_narrow(emu, fmt, 3));
    emu.kernel_log_line(line);
    call.set_return(emu, 0);
}

fn vDbgPrintEx(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let fmt = emu.maps.read_string(api_try!(emu, call.arg(emu, 2)));
    let va = api_try!(emu, call.arg(emu, 3));
    let line = api_try!(
        emu,
        format_args(
            emu,
            &fmt,
            &mut ArgSource::VaList { base: va, index: 0 },
            false,
        )
    );
    emu.kernel_log_line(line);
    call.set_return(emu, 0);
}

fn vDbgPrintExWithPrefix(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let prefix = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let fmt = emu.maps.read_string(api_try!(emu, call.arg(emu, 3)));
    let va = api_try!(emu, call.arg(emu, 4));
    let line = api_try!(
        emu,
        format_args(
            emu,
            &fmt,
            &mut ArgSource::VaList { base: va, index: 0 },
            false,
        )
    );
    emu.kernel_log_line(format!("{}{}", prefix, line));
    call.set_return(emu, 0);
}

fn AslLogCallPrintf(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let msg = emu.maps.read_string(api_try!(emu, call.arg(emu, 3)));
    emu.kernel_log_line(format!("ACPI: {}", msg));
    call.set_return(emu, 0);
}

fn debug_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    emu.kernel_log_line(format!("{} called", symbol));
    call.set_return(emu, 0);
}

fn memcpy(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    checked_copy(emu, dst, src, len);
    call.set_return(emu, dst);
}

fn secure_copy(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let capacity = api_try!(emu, call.arg(emu, 1));
    let src = api_try!(emu, call.arg(emu, 2));
    let len = api_try!(emu, call.arg(emu, 3));
    let error = if len == 0 {
        0
    } else if dst == 0 || src == 0 {
        EINVAL
    } else if len > capacity {
        ERANGE
    } else {
        if !usize::try_from(len).is_ok_and(|n| emu.maps.can_access_range(src, n, Permission::READ))
            || !emu.maps.validate_write_range(dst, len)
        {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        checked_copy(emu, dst, src, len);
        0
    };
    if error != 0 && symbol == "memcpy_s" && dst != 0 && capacity != 0 {
        if !emu.maps.validate_write_range(dst, capacity) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        checked_fill(emu, dst, 0, capacity);
    }
    call.set_return(emu, error as u64);
}

fn memset(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    match symbol {
        "memset" => {
            let byte = api_try!(emu, call.arg_u8(emu, 1));
            let len = api_try!(emu, call.arg(emu, 2));
            checked_fill(emu, dst, byte, len);
        }
        "RtlFillBytes" => {
            let len = api_try!(emu, call.arg(emu, 1));
            let byte = api_try!(emu, call.arg_u8(emu, 2));
            checked_fill(emu, dst, byte, len);
        }
        _ => {
            let len = api_try!(emu, call.arg(emu, 1));
            let pat = api_try!(emu, call.arg(emu, 2));
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
    call.set_return(emu, dst);
}

fn memcmp(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let a = api_try!(emu, call.arg(emu, 0));
    let b = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
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
    call.set_return(emu, r);
}

fn memchr(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let found = (0..len)
        .find(|i| emu.maps.read_byte(p + i) == Some(c))
        .map(|i| p + i)
        .unwrap_or(0);
    call.set_return(emu, found);
}

fn unit_address(emu: &mut Emu, base: u64, index: u64, wide: bool) -> Option<u64> {
    let address = index
        .checked_mul(if wide { 2 } else { 1 })
        .and_then(|offset| base.checked_add(offset));
    if address.is_none() {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
    address
}

fn read_unit(emu: &mut Emu, base: u64, index: u64, wide: bool) -> Option<u16> {
    let address = unit_address(emu, base, index, wide)?;
    let width = if wide { 2 } else { 1 };
    let pc = emu.pc();
    emu.kernel_guard_range(pc, address, width, false);
    emu.kernel_guard_access(pc, address, width as u32, false);
    let value = if wide {
        read_array::<2>(emu, address).map(u16::from_le_bytes)
    } else {
        emu.maps.read_byte(address).map(u16::from)
    };
    if value.is_none() {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
    value
}

fn string_units(emu: &mut Emu, base: u64, limit: u64, wide: bool) -> Option<Vec<u16>> {
    let mut units = Vec::new();
    for index in 0..limit {
        let value = read_unit(emu, base, index, wide)?;
        if value == 0 {
            break;
        }
        units.push(value);
    }
    Some(units)
}

fn write_units(emu: &mut Emu, base: u64, units: &[u16], wide: bool) {
    let bytes: Vec<u8> = if wide {
        units.iter().flat_map(|unit| unit.to_le_bytes()).collect()
    } else {
        units.iter().map(|&unit| unit as u8).collect()
    };
    checked_copy_bytes(emu, base, &bytes);
}

fn string_length(emu: &mut Emu, symbol: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let limit = if symbol.contains("nlen") {
        api_try!(emu, call.arg(emu, 1))
    } else {
        u64::MAX
    };
    if address == 0 && symbol.ends_with("_s") {
        call.set_return(emu, 0);
        return;
    }
    let mut length = 0;
    while length < limit {
        let Some(unit) = read_unit(emu, address, length, wide) else {
            return;
        };
        if unit == 0 {
            break;
        }
        length += 1;
    }
    call.set_return(emu, length);
}

fn compare_string_units(emu: &mut Emu, symbol: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let limited = symbol.contains("ncmp") || symbol.contains("nicmp");
    let limit = if limited {
        api_try!(emu, call.arg(emu, 2))
    } else {
        u64::MAX
    };
    let insensitive = symbol.contains("icmp") || symbol.contains("cmpi");
    for index in 0..limit {
        let Some(mut a) = read_unit(emu, left, index, wide) else {
            return;
        };
        let Some(mut b) = read_unit(emu, right, index, wide) else {
            return;
        };
        if insensitive {
            if a <= u8::MAX as u16 {
                a = (a as u8).to_ascii_lowercase() as u16;
            }
            if b <= u8::MAX as u16 {
                b = (b as u8).to_ascii_lowercase() as u16;
            }
        }
        if a != b || a == 0 {
            call.set_return(emu, cmp_result(a.cmp(&b)));
            return;
        }
    }
    call.set_return(emu, 0);
}

fn string_copy(emu: &mut Emu, symbol: &str, wide: bool, append: bool, limited: bool) {
    let call = WinApi(Stdcall);
    let destination = api_try!(emu, call.arg(emu, 0));
    let secure = symbol.ends_with("_s");
    let capacity = if secure {
        api_try!(emu, call.arg(emu, 1))
    } else {
        u64::MAX
    };
    let source_index = if secure { 2 } else { 1 };
    let source = api_try!(emu, call.arg(emu, source_index));
    let count = if limited {
        api_try!(emu, call.arg(emu, source_index + 1))
    } else {
        u64::MAX
    };
    let width = if wide { 2 } else { 1 };
    if secure && limited && count == 0 && destination == 0 && capacity == 0 {
        call.set_return(emu, 0);
        return;
    }
    if secure && (destination == 0 || capacity == 0 || (source == 0 && (!limited || count != 0))) {
        if destination != 0 && capacity != 0 {
            checked_fill(emu, destination, 0, width);
        }
        call.set_return(emu, EINVAL as u64);
        return;
    }
    let destination_length = if append {
        let Some(units) = string_units(emu, destination, capacity, wide) else {
            return;
        };
        units.len() as u64
    } else {
        0
    };
    if secure && destination_length == capacity {
        checked_fill(emu, destination, 0, width);
        call.set_return(emu, EINVAL as u64);
        return;
    }
    let remaining = capacity - destination_length;
    let scan_limit = if secure { count.min(remaining) } else { count };
    let Some(mut units) = string_units(emu, source, scan_limit, wide) else {
        return;
    };
    let mut result = 0;
    if secure && units.len() as u64 >= remaining {
        if limited && count == _TRUNCATE {
            units.truncate((remaining - 1) as usize);
            result = STRUNCATE;
        } else {
            checked_fill(emu, destination, 0, width);
            call.set_return(emu, ERANGE as u64);
            return;
        }
    }
    let Some(output) = unit_address(emu, destination, destination_length, wide) else {
        return;
    };
    let copied = units.len() as u64;
    if secure || append || !limited {
        units.push(0);
    }
    write_units(emu, output, &units, wide);
    if !secure && !append && limited && copied < count {
        let Some(padding) = unit_address(emu, output, copied, wide) else {
            return;
        };
        let Some(bytes) = (count - copied).checked_mul(width) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        };
        checked_fill(emu, padding, 0, bytes);
    }
    call.set_return(emu, if secure { result as u64 } else { destination });
}

fn find_string_unit(emu: &mut Emu, symbol: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg(emu, 0));
    let target = if wide {
        api_try!(emu, call.arg_u16(emu, 1))
    } else {
        api_try!(emu, call.arg_u8(emu, 1)) as u16
    };
    let reverse = symbol.ends_with("rchr");
    let mut found = 0;
    for index in 0..u64::MAX {
        let Some(value) = read_unit(emu, base, index, wide) else {
            return;
        };
        if value == target {
            let Some(address) = unit_address(emu, base, index, wide) else {
                return;
            };
            found = address;
            if !reverse {
                break;
            }
        }
        if value == 0 {
            break;
        }
    }
    call.set_return(emu, found);
}

fn strlen(emu: &mut Emu, symbol: &str) {
    string_length(emu, symbol, false);
}

fn strnlen(emu: &mut Emu, symbol: &str) {
    string_length(emu, symbol, false);
}

fn strcmp(emu: &mut Emu, symbol: &str) {
    compare_string_units(emu, symbol, false);
}

fn strcpy(emu: &mut Emu, symbol: &str) {
    string_copy(emu, symbol, false, false, false);
}

fn strncpy(emu: &mut Emu, symbol: &str) {
    string_copy(emu, symbol, false, false, true);
}

fn strcat(emu: &mut Emu, symbol: &str) {
    string_copy(emu, symbol, false, true, symbol.contains("ncat"));
}

fn strchr(emu: &mut Emu, symbol: &str) {
    find_string_unit(emu, symbol, false);
}

fn strstr(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(p);
    let needle = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    let r = s.find(&needle).map(|i| p + i as u64).unwrap_or(0);
    call.set_return(emu, r);
}

fn _strlwr(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(p);
    let t = if symbol.contains("lwr") {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    };
    emu.maps.write_string(p, &t);
    call.set_return(emu, p);
}

fn toupper(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let c = api_try!(emu, call.arg_u8(emu, 0)) as char;
    let r = if symbol.contains("upper") {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    };
    call.set_return(emu, r as u64);
}

fn isdigit(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let c = api_try!(emu, call.arg_u8(emu, 0)) as char;
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
    call.set_return(emu, r as u64);
}

fn atoi(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let (end_ptr, base) =
        if symbol.starts_with("atoi") || symbol.starts_with("atol") || symbol == "_atoi64" {
            (0, 10)
        } else {
            (
                api_try!(emu, call.arg(emu, 1)),
                api_try!(emu, call.arg_u32(emu, 2)),
            )
        };
    let (v, used) = parse_int(&s, base);
    if end_ptr != 0 {
        emu.maps
            .write_qword(end_ptr, api_try!(emu, call.arg(emu, 0)) + used as u64);
    }
    call.set_return(emu, v as u64);
}

fn _itoa(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let v = api_try!(emu, call.arg(emu, 0));
    let (buf, radix) = if symbol.ends_with("_s") {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg_u32(emu, 3)),
        )
    } else {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg_u32(emu, 2)),
        )
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
    call.set_return(emu, if symbol.ends_with("_s") { 0 } else { buf });
}

fn wcslen(emu: &mut Emu, symbol: &str) {
    string_length(emu, symbol, true);
}

fn wcsnlen(emu: &mut Emu, symbol: &str) {
    string_length(emu, symbol, true);
}

fn wcscmp(emu: &mut Emu, symbol: &str) {
    compare_string_units(emu, symbol, true);
}

fn wcscpy(emu: &mut Emu, symbol: &str) {
    string_copy(emu, symbol, true, false, false);
}

fn wcsncpy(emu: &mut Emu, symbol: &str) {
    string_copy(emu, symbol, true, false, true);
}

fn wcscat(emu: &mut Emu, symbol: &str) {
    string_copy(emu, symbol, true, true, symbol.contains("ncat"));
}

fn wcschr(emu: &mut Emu, symbol: &str) {
    find_string_unit(emu, symbol, true);
}

fn wcsstr(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_wide_string(p);
    let needle = emu.maps.read_wide_string(api_try!(emu, call.arg(emu, 1)));
    let r = s
        .find(&needle)
        .map(|i| p + s[..i].encode_utf16().count() as u64 * 2)
        .unwrap_or(0);
    call.set_return(emu, r);
}

fn _wcslwr(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_wide_string(p);
    let t = if symbol.contains("lwr") {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    };
    emu.maps.write_wide_string(p, &t);
    call.set_return(emu, p);
}

fn towupper(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let c = char::from_u32(api_try!(emu, call.arg_u16(emu, 0)) as u32).unwrap_or('\0');
    let r = if symbol == "towupper" {
        c.to_uppercase().next()
    } else {
        c.to_lowercase().next()
    };
    call.set_return(emu, r.unwrap_or(c) as u64);
}

fn iswdigit(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let c = char::from_u32(api_try!(emu, call.arg_u16(emu, 0)) as u32).unwrap_or('\0');
    let r = match symbol {
        "iswdigit" => c.is_ascii_digit(),
        "iswspace" => c.is_whitespace(),
        "iswalpha" => c.is_alphabetic(),
        "iswalnum" => c.is_alphanumeric(),
        _ => c.is_ascii_hexdigit(),
    };
    call.set_return(emu, r as u64);
}

fn _wtoi(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let s = emu.maps.read_wide_string(api_try!(emu, call.arg(emu, 0)));
    let (end_ptr, base) = if symbol.starts_with("_wto") {
        (0, 10)
    } else {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg_u32(emu, 2)),
        )
    };
    let (v, used) = parse_int(&s, base);
    if end_ptr != 0 {
        emu.maps
            .write_qword(end_ptr, api_try!(emu, call.arg(emu, 0)) + used as u64 * 2);
    }
    call.set_return(emu, v as u64);
}

fn mbstowcs(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    if symbol == "mbstowcs" {
        let s: String = emu.maps.read_string(src).chars().take(n).collect();
        if dst != 0 {
            emu.maps.write_wide_string(dst, &s);
        }
        call.set_return(emu, s.encode_utf16().count() as u64);
    } else {
        let s: String = emu.maps.read_wide_string(src).chars().take(n).collect();
        if dst != 0 {
            emu.maps.write_string(dst, &s);
        }
        call.set_return(emu, s.len() as u64);
    }
}

fn sprintf(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let (count, fmt_idx) = match symbol {
        "sprintf" => (u64::MAX, 1),
        "_snprintf" => (api_try!(emu, call.arg(emu, 1)), 2),
        "sprintf_s" => (api_try!(emu, call.arg(emu, 1)), 2),
        _ => (
            api_try!(emu, call.arg(emu, 2)).min(api_try!(emu, call.arg(emu, 1))),
            3,
        ),
    };
    let fmt = emu.maps.read_string(api_try!(emu, call.arg(emu, fmt_idx)));
    let s = api_try!(
        emu,
        format_args(emu, &fmt, &mut ArgSource::Call(fmt_idx + 1), false)
    );
    let r = write_narrow_result(emu, buf, count, &s);
    call.set_return(emu, r);
}

fn vsprintf(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let (count, fmt_idx) = match symbol {
        "vsprintf" => (u64::MAX, 1),
        "_vsnprintf" | "vsprintf_s" => (api_try!(emu, call.arg(emu, 1)), 2),
        _ => (
            api_try!(emu, call.arg(emu, 2)).min(api_try!(emu, call.arg(emu, 1))),
            3,
        ),
    };
    let fmt = emu.maps.read_string(api_try!(emu, call.arg(emu, fmt_idx)));
    let va = api_try!(emu, call.arg(emu, fmt_idx + 1));
    let s = api_try!(
        emu,
        format_args(
            emu,
            &fmt,
            &mut ArgSource::VaList { base: va, index: 0 },
            false,
        )
    );
    let r = write_narrow_result(emu, buf, count, &s);
    call.set_return(emu, r);
}

fn swprintf(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let (count, fmt_idx) = match symbol {
        "swprintf" => (u64::MAX, 1),
        "_snwprintf" | "swprintf_s" => (api_try!(emu, call.arg(emu, 1)), 2),
        _ => (
            api_try!(emu, call.arg(emu, 2)).min(api_try!(emu, call.arg(emu, 1))),
            3,
        ),
    };
    let fmt = emu
        .maps
        .read_wide_string(api_try!(emu, call.arg(emu, fmt_idx)));
    let s = api_try!(
        emu,
        format_args(emu, &fmt, &mut ArgSource::Call(fmt_idx + 1), true)
    );
    let r = write_wide_result(emu, buf, count, &s);
    call.set_return(emu, r);
}

fn vswprintf(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let (count, fmt_idx) = match symbol {
        "vswprintf" => (u64::MAX, 1),
        "_vsnwprintf" | "vswprintf_s" => (api_try!(emu, call.arg(emu, 1)), 2),
        _ => (
            api_try!(emu, call.arg(emu, 2)).min(api_try!(emu, call.arg(emu, 1))),
            3,
        ),
    };
    let fmt = emu
        .maps
        .read_wide_string(api_try!(emu, call.arg(emu, fmt_idx)));
    let va = api_try!(emu, call.arg(emu, fmt_idx + 1));
    let s = api_try!(
        emu,
        format_args(
            emu,
            &fmt,
            &mut ArgSource::VaList { base: va, index: 0 },
            true,
        )
    );
    let r = write_wide_result(emu, buf, count, &s);
    call.set_return(emu, r);
}

fn RtlStringCbPrintfA(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let wide = symbol.ends_with('W');
    let count = if symbol.contains("Cb") && wide {
        size / 2
    } else {
        size
    };
    let fmt = if wide {
        emu.maps.read_wide_string(api_try!(emu, call.arg(emu, 2)))
    } else {
        emu.maps.read_string(api_try!(emu, call.arg(emu, 2)))
    };
    let s = api_try!(emu, format_args(emu, &fmt, &mut ArgSource::Call(3), wide));
    let r = if wide {
        write_wide_result(emu, buf, count, &s)
    } else {
        write_narrow_result(emu, buf, count, &s)
    };
    call.set_return(
        emu,
        if r == u64::MAX {
            STATUS_BUFFER_OVERFLOW
        } else {
            STATUS_SUCCESS
        },
    );
}

fn RtlStringCbCopyA(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let wide = symbol.ends_with('W');
    let count = if symbol.contains("Cb") && wide {
        size / 2
    } else {
        size
    };
    let s = if wide {
        emu.maps.read_wide_string(api_try!(emu, call.arg(emu, 2)))
    } else {
        emu.maps.read_string(api_try!(emu, call.arg(emu, 2)))
    };
    let r = if wide {
        write_wide_result(emu, buf, count, &s)
    } else {
        write_narrow_result(emu, buf, count, &s)
    };
    call.set_return(
        emu,
        if r == u64::MAX {
            STATUS_BUFFER_OVERFLOW
        } else {
            STATUS_SUCCESS
        },
    );
}

fn __security_check_cookie(emu: &mut Emu, _symbol: &str) {
    let rcx = emu.regs().rcx;
    if let Some(cookie) = emu.read_security_cookie()
        && rcx != cookie
    {
        log::info!("--- STACK COOKIE CHECK FAILURE (intercepted) ---");
        log::info!("  rcx=0x{:x}  __security_cookie=0x{:x}", rcx, cookie);
        emu.kernel_abort(format!(
            "__security_check_cookie: mismatch 0x{:x} vs 0x{:x}",
            rcx, cookie
        ));
    }
}

fn abort_call(emu: &mut Emu, symbol: &str) {
    emu.kernel_abort(format!("{} called", symbol));
}

fn rand(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let v = emu.pos.wrapping_mul(1_103_515_245).wrapping_add(12345);
    call.set_return(emu, (v >> 16) & 0x7fff);
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
