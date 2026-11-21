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
use crate::api::unix::{MemoryAccess, MemoryError, api_try, memory};
use crate::emu::Emu;
use crate::kernel::linux::{
    mm::{guard_range, guarded_copy},
    printk,
};
use crate::kernel::{HandlerRegistry, kernel_handlers};
use crate::syscall::linux::constants::errors::{E2BIG, EINVAL, ERANGE};

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
        "snprintf" | "scnprintf" | "sprintf" => formatted_output,

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
    api_try!(emu, guarded_copy(emu, dst, src, len));
    call.set_return(emu, dst);
}

fn memset(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let byte = api_try!(emu, call.arg_u8(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    guard_range(emu, dst, len, true);
    api_try!(emu, memory::fill(emu, dst, byte, len));
    call.set_return(emu, dst);
}

fn memcmp(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let a = api_try!(emu, call.arg(emu, 0));
    let b = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    guard_range(emu, a, len, false);
    guard_range(emu, b, len, false);
    let result = api_try!(emu, memory::compare_bytes(emu, a, b, len));
    call.set_return(emu, result as u64);
}

fn strlen(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let n = api_try!(emu, memory::length(emu, p, u64::MAX));
    guard_range(emu, p, n + 1, false);
    call.set_return(emu, n);
}

fn strnlen(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let p = api_try!(emu, call.arg(emu, 0));
    let max = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, memory::length(emu, p, max));
    guard_range(emu, p, n + u64::from(n < max), false);
    call.set_return(emu, n);
}

fn strcmp(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let count = if matches!(symbol, "strncmp" | "strncasecmp") {
        api_try!(emu, call.arg(emu, 2))
    } else {
        u64::MAX
    };
    let fold = matches!(symbol, "strcasecmp" | "strncasecmp");
    let result = api_try!(emu, memory::compare_strings(emu, left, right, count, fold));
    call.set_return(emu, result as u64);
}

fn strcpy(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, memory::length(emu, src, u64::MAX));
    api_try!(emu, guarded_copy(emu, dst, src, len + 1));
    call.set_return(emu, dst);
}

fn strncpy(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg(emu, 2));
    let counted = symbol.contains("strscpy");
    if counted && capacity == 0 {
        call.set_return(emu, E2BIG);
        return;
    }
    let limit = if symbol == "strlcpy" {
        u64::MAX
    } else {
        capacity
    };
    let length = api_try!(emu, memory::length(emu, source, limit));
    guard_range(emu, source, length + u64::from(length < limit), false);
    let copied = if symbol == "strncpy" {
        length
    } else {
        length.min(capacity.saturating_sub(1))
    };
    let write_size = if symbol == "strncpy" || symbol == "strscpy_pad" {
        capacity
    } else if capacity == 0 {
        0
    } else {
        copied + 1
    };
    guard_range(emu, destination, write_size, true);
    api_try!(
        emu,
        memory::range(emu, destination, write_size, MemoryAccess::Write)
    );
    api_try!(emu, memory::copy(emu, destination, source, copied));
    if copied < write_size {
        api_try!(
            emu,
            memory::fill(emu, destination + copied, 0, write_size - copied)
        );
    }
    let result = match symbol {
        "strncpy" => destination,
        "strlcpy" => length,
        _ if length == capacity => E2BIG,
        _ => copied,
    };
    call.set_return(emu, result);
}

fn strcat(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let limit = if symbol == "strncat" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        u64::MAX
    };
    let prefix = api_try!(emu, memory::length(emu, destination, u64::MAX));
    let suffix = api_try!(emu, memory::length(emu, source, limit));
    guard_range(emu, destination, prefix + 1, false);
    guard_range(emu, source, suffix + u64::from(suffix < limit), false);
    guard_range(emu, destination + prefix, suffix + 1, true);
    api_try!(
        emu,
        memory::terminated_copy(emu, destination + prefix, source, suffix)
    );
    call.set_return(emu, destination);
}

fn strchr(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let address = api_try!(emu, call.arg(emu, 0));
    let byte = api_try!(emu, call.arg_u8(emu, 1));
    let found = api_try!(
        emu,
        memory::find_character(emu, address, byte, symbol == "strrchr")
    );
    call.set_return(emu, found);
}

fn strstr(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let needle = api_try!(emu, call.arg(emu, 1));
    let found = api_try!(emu, memory::find_string(emu, source, needle));
    call.set_return(emu, found);
}

fn formatted_output(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let bounded = symbol != "sprintf";
    let limit = if bounded {
        api_try!(emu, call.arg(emu, 1))
    } else {
        u64::MAX
    };
    let format_index = 1 + usize::from(bounded);
    let fmt = api_try!(emu, call.arg(emu, format_index));
    let text = api_try!(emu, printk::format_bytes(emu, fmt, format_index + 1));
    let required = text.len() as u64;
    let written = required.min(limit.saturating_sub(1));
    if limit != 0 {
        guard_range(emu, dst, written + 1, true);
        api_try!(
            emu,
            memory::range(emu, dst, written + 1, MemoryAccess::Write)
        );
        api_try!(emu, memory::write(emu, dst, &text[..written as usize]));
        api_try!(emu, memory::write(emu, dst + written, &[0]));
    }
    call.set_return(
        emu,
        if symbol == "scnprintf" {
            written
        } else {
            required
        },
    );
}

struct Digits {
    value: u64,
    end: u64,
    overflow: bool,
    any: bool,
}

fn digits(emu: &Emu, mut at: u64, mut base: u32) -> Result<Digits, MemoryError> {
    let mut result = Digits {
        value: 0,
        end: at,
        overflow: false,
        any: false,
    };
    if base != 0 && !(2..=16).contains(&base) {
        return Ok(result);
    }
    let mut byte = memory::byte_at(emu, at, 0)?;
    if byte == b'0' && matches!(base, 0 | 16) {
        let hex = matches!(memory::byte_at(emu, at, 1)?, b'x' | b'X');
        if hex && (base == 16 || memory::byte_at(emu, at, 2)?.is_ascii_hexdigit()) {
            base = 16;
            byte = memory::byte_at(emu, at, 2)?;
            at += 2;
        }
    }
    if base == 0 {
        base = if byte == b'0' { 8 } else { 10 };
    }
    while let Some(digit) = char::from(byte).to_digit(base) {
        let (product, multiplication_overflow) = result.value.overflowing_mul(base.into());
        let (value, addition_overflow) = product.overflowing_add(digit.into());
        result.value = value;
        result.overflow |= multiplication_overflow || addition_overflow;
        result.any = true;
        byte = memory::byte_at(emu, at, 1)?;
        at += 1;
    }
    result.end = at;
    Ok(result)
}

fn simple_strtoul(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let end = api_try!(emu, call.arg(emu, 1));
    let base = api_try!(emu, call.arg_u32(emu, 2));
    let negative =
        symbol == "simple_strtol" && api_try!(emu, memory::byte_at(emu, source, 0)) == b'-';
    let parsed = api_try!(
        emu,
        digits(emu, source.saturating_add(u64::from(negative)), base)
    );
    if end != 0 {
        api_try!(emu, memory::write(emu, end, &parsed.end.to_le_bytes()));
    }
    call.set_return(
        emu,
        if negative {
            parsed.value.wrapping_neg()
        } else {
            parsed.value
        },
    );
}

fn kstrtoint(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let base = api_try!(emu, call.arg_u32(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let signed = matches!(symbol, "kstrtoint" | "kstrtol" | "kstrtoll");
    let narrow = matches!(symbol, "kstrtoint" | "kstrtouint" | "kstrtou32");
    let first = api_try!(emu, memory::byte_at(emu, source, 0));
    let negative = signed && first == b'-';
    let skip = first == b'+' || negative;
    let parsed = api_try!(
        emu,
        digits(emu, source.saturating_add(u64::from(skip)), base)
    );
    if parsed.overflow {
        call.set_return(emu, ERANGE);
        return;
    }
    let next = api_try!(emu, memory::byte_at(emu, parsed.end, 0));
    if !parsed.any
        || !(next == 0 || next == b'\n' && api_try!(emu, memory::byte_at(emu, parsed.end, 1)) == 0)
    {
        call.set_return(emu, EINVAL);
        return;
    }
    let value = i128::from(parsed.value) * if negative { -1 } else { 1 };
    let valid = match (signed, narrow) {
        (true, true) => i32::try_from(value).is_ok(),
        (false, true) => u32::try_from(value).is_ok(),
        (true, false) => i64::try_from(value).is_ok(),
        (false, false) => true,
    };
    if !valid {
        call.set_return(emu, ERANGE);
        return;
    }
    let size = if narrow {
        size_of::<u32>()
    } else {
        size_of::<u64>()
    };
    api_try!(
        emu,
        memory::write(emu, output, &value.to_le_bytes()[..size])
    );
    call.set_return(emu, 0);
}
