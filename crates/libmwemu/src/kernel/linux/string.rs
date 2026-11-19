//! `lib/string.c` and the `kstrto*` / `*printf` helpers.
//!
//! Kernel modules do not link a libc: `memcpy`, `strlen` and friends are
//! ordinary exported kernel symbols, so they arrive here as imports. Routing
//! them through the guard matters as much as implementing them — a `memcpy()`
//! into a freed object is a use-after-free that no instruction-level check
//! would ever see, because the copy happens inside the kernel, not in the
//! driver's own code.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::linux::printk;
use crate::kernel::{HandlerRegistry, kernel_handlers};

/// Copy `len` bytes, reporting either end that lands in a freed or
/// out-of-bounds chunk.
fn checked_copy(emu: &mut Emu, dst: u64, src: u64, len: u64) {
    if len == 0 {
        return;
    }
    let rip = emu.pc();
    // Range-aware: flags a copy whose length overruns the destination chunk's
    // requested size even when the tail lands past the mapped bucket.
    emu.kernel_guard_range(rip, src, len, false);
    emu.kernel_guard_range(rip, dst, len, true);
    emu.kernel_guard_access(rip, src, len as u32, false);
    emu.kernel_guard_access(rip, dst, len as u32, true);

    let mut buf = vec![0u8; len as usize];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
    }
    emu.maps.write_bytes(dst, &buf);
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        "memcpy" | "__memcpy" | "memmove" | "__memmove" | "memcpy_toio" | "memcpy_fromio" => memcpy,
        "memset" | "__memset" | "memset_io" => memset,
        "memcmp" | "bcmp" => memcmp,
        "strlen" => strlen,
        "strnlen" => strnlen,
        "strcmp" | "strncmp" | "strcasecmp" | "strncasecmp" => strcmp,
        "strcpy" => strcpy,
        "strncpy" | "strscpy" | "sized_strscpy" | "strlcpy" | "strscpy_pad" => strncpy,
        "strcat" | "strncat" => strcat,
        "strchr" | "strrchr" => strchr,
        "strstr" => strstr,

        // --- formatted output ------------------------------------------------
        "snprintf" | "scnprintf" => snprintf,
        "sprintf" => sprintf,

        // --- string to number -------------------------------------------------
        "simple_strtoul" | "simple_strtol" | "simple_strtoull" => simple_strtoul,
        "kstrtoint" | "kstrtol" | "kstrtoul" | "kstrtouint" | "kstrtou32" | "kstrtou64"
        | "kstrtoll" | "kstrtoull" => kstrtoint,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn memcpy(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    checked_copy(emu, dst, src, len);
    call.set_return(emu, dst);
}

fn memset(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let byte = api_try!(emu, call.arg_u8(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    if len > 0 {
        let rip = emu.pc();
        emu.kernel_guard_access(rip, dst, len as u32, true);
        emu.kernel_guard_access(rip, dst + len - 1, 1, true);
        emu.maps.write_bytes(dst, &vec![byte; len as usize]);
    }
    call.set_return(emu, dst);
}

fn memcmp(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let a = api_try!(emu, call.arg(emu, 0));
    let b = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let rip = emu.pc();
    emu.kernel_guard_access(rip, a, len as u32, false);
    emu.kernel_guard_access(rip, b, len as u32, false);
    let mut result: i64 = 0;
    for i in 0..len {
        let x = emu.maps.read_byte(a + i).unwrap_or(0) as i64;
        let y = emu.maps.read_byte(b + i).unwrap_or(0) as i64;
        if x != y {
            result = x - y;
            break;
        }
    }
    call.set_return(emu, result as u64);
}

fn strlen(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let rip = emu.pc();
    emu.kernel_guard_access(rip, p, 1, false);
    let n = emu.maps.read_string(p).len() as u64;
    call.set_return(emu, n);
}

fn strnlen(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let max = api_try!(emu, call.arg(emu, 1));
    let n = (emu.maps.read_string(p).len() as u64).min(max);
    call.set_return(emu, n);
}

fn strcmp(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let a = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let b = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    let (a, b) = if symbol.starts_with("strn") {
        let n = api_try!(emu, call.arg(emu, 2)) as usize;
        (
            a.chars().take(n).collect::<String>(),
            b.chars().take(n).collect::<String>(),
        )
    } else {
        (a, b)
    };
    let (a, b) = if symbol.contains("case") {
        (a.to_lowercase(), b.to_lowercase())
    } else {
        (a, b)
    };
    let r: i64 = match a.cmp(&b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    call.set_return(emu, r as u64);
}

fn strcpy(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_string(src);
    checked_copy(emu, dst, src, s.len() as u64 + 1);
    call.set_return(emu, dst);
}

fn strncpy(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let max = api_try!(emu, call.arg(emu, 2));
    let mut s = emu.maps.read_string(src);
    s.truncate(max.saturating_sub(1) as usize);
    let rip = emu.pc();
    emu.kernel_guard_access(rip, dst, s.len() as u32 + 1, true);
    emu.maps.write_string(dst, &s);
    // strscpy returns the length copied, strncpy/strlcpy return dest.
    let ret = if symbol.contains("strscpy") {
        s.len() as u64
    } else {
        dst
    };
    call.set_return(emu, ret);
}

fn strcat(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let existing = emu.maps.read_string(dst);
    let s = emu.maps.read_string(src);
    let end = dst + existing.len() as u64;
    let rip = emu.pc();
    emu.kernel_guard_access(rip, end, s.len() as u32 + 1, true);
    emu.maps.write_string(end, &s);
    call.set_return(emu, dst);
}

fn strchr(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1)) as char;
    let s = emu.maps.read_string(p);
    let found = if symbol == "strchr" {
        s.find(c)
    } else {
        s.rfind(c)
    };
    call.set_return(emu, found.map(|i| p + i as u64).unwrap_or(0));
}

fn strstr(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let hay = emu.maps.read_string(p);
    let needle = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    call.set_return(emu, hay.find(&needle).map(|i| p + i as u64).unwrap_or(0));
}

fn snprintf(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let max = api_try!(emu, call.arg(emu, 1));
    let fmt = api_try!(emu, call.arg(emu, 2));
    let mut s = api_try!(emu, printk::format(emu, fmt, 3));
    s.truncate(max.saturating_sub(1) as usize);
    let rip = emu.pc();
    emu.kernel_guard_access(rip, dst, s.len() as u32 + 1, true);
    emu.maps.write_string(dst, &s);
    call.set_return(emu, s.len() as u64);
}

fn sprintf(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let fmt = api_try!(emu, call.arg(emu, 1));
    let s = api_try!(emu, printk::format(emu, fmt, 2));
    let rip = emu.pc();
    emu.kernel_guard_access(rip, dst, s.len() as u32 + 1, true);
    emu.maps.write_string(dst, &s);
    call.set_return(emu, s.len() as u64);
}

fn simple_strtoul(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let base = api_try!(emu, call.arg_u32(emu, 2));
    let base = if base == 0 { 10 } else { base };
    let trimmed: String = s
        .chars()
        .take_while(|c| c.is_digit(base) || *c == '-')
        .collect();
    call.set_return(emu, i64::from_str_radix(&trimmed, base).unwrap_or(0) as u64);
}

fn kstrtoint(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let s = emu
        .maps
        .read_string(api_try!(emu, call.arg(emu, 0)))
        .trim()
        .to_string();
    let base = api_try!(emu, call.arg_u32(emu, 1));
    let base = if base == 0 { 10 } else { base };
    let out = api_try!(emu, call.arg(emu, 2));
    match i64::from_str_radix(&s, base) {
        Ok(v) => {
            if symbol.ends_with("int") || symbol.ends_with("u32") {
                emu.maps.write_dword(out, v as u32);
            } else {
                emu.maps.write_qword(out, v as u64);
            }
            call.set_return(emu, 0);
        }
        // -EINVAL
        Err(_) => call.set_return(emu, (-22i64) as u64),
    }
}
