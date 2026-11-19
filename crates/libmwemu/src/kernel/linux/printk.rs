//! `printk` and the kernel's `vsnprintf` family.
//!
//! The formatter is shared with `snprintf`/`scnprintf`/`kasprintf`, because in
//! the kernel they are all the same engine — and because a driver's log lines
//! are usually the fastest way to understand what path an emulated run took.

use crate::api::abi::unix::ArgumentError;
use crate::api::abi::unix::{UnixApi, VariadicConvention};
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::KernelOs;
use crate::kernel::{HandlerRegistry, kernel_handlers};

/// Render a kernel format string, pulling variadic arguments starting at
/// argument index `first_arg`.
///
/// Supports the conversions a driver realistically uses: `%d %i %u %x %X %o
/// %c %s %p<suffix> %%`, with `l`/`ll`/`z`/`h` length modifiers and
/// width/precision/flags skipped rather than honoured. Anything unrecognised is
/// emitted verbatim so a log line is never silently truncated.
pub fn format(emu: &mut Emu, fmt_addr: u64, first_arg: usize) -> Result<String, ArgumentError> {
    let fmt = emu.maps.read_string(fmt_addr);
    // Kernel log levels are encoded as SOH + digit at the head of the string.
    let fmt = fmt.strip_prefix('\u{1}').map_or(fmt.as_str(), |rest| {
        rest.strip_prefix(|c: char| c.is_ascii_digit() || c == 'c')
            .unwrap_or(rest)
    });

    let mut out = String::new();
    let call = UnixApi;
    let convention = if matches!(
        emu.kernel.as_ref().map(|kernel| kernel.os),
        Some(KernelOs::MacOS)
    ) {
        VariadicConvention::Darwin
    } else {
        VariadicConvention::SystemV
    };
    let mut number = 0;
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

        // Flags, width and precision: consumed but not applied. Driver logs
        // rarely depend on the padding, and honouring it would not change what
        // an analyst learns from the line.
        while matches!(chars.peek(), Some('-' | '+' | ' ' | '#' | '0')) {
            chars.next();
        }
        while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
            chars.next();
        }
        if chars.peek() == Some(&'.') {
            chars.next();
            while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
                chars.next();
            }
        }
        while matches!(chars.peek(), Some('l' | 'h' | 'z' | 'j' | 't' | 'q')) {
            chars.next();
        }

        let Some(conv) = chars.next() else { break };
        let mut next = |emu: &Emu| -> Result<u64, ArgumentError> {
            let value = call.integer_vararg(emu, first_arg, number, convention)?;
            number += 1;
            Ok(value)
        };

        match conv {
            'd' | 'i' => out.push_str(&(next(emu)? as i64).to_string()),
            'u' => out.push_str(&next(emu)?.to_string()),
            'x' => out.push_str(&format!("{:x}", next(emu)?)),
            'X' => out.push_str(&format!("{:X}", next(emu)?)),
            'o' => out.push_str(&format!("{:o}", next(emu)?)),
            'c' => out.push(next(emu)? as u8 as char),
            's' => {
                let ptr = next(emu)?;
                if ptr == 0 {
                    out.push_str("(null)");
                } else {
                    out.push_str(&emu.maps.read_string(ptr));
                }
            }
            'p' => {
                // `%pK`, `%px`, `%pS`, `%pI4`, … — the suffix selects a
                // rendering, not another argument.
                while matches!(chars.peek(), Some(c) if c.is_ascii_alphanumeric()) {
                    chars.next();
                }
                out.push_str(&format!("0x{:x}", next(emu)?));
            }
            other => {
                out.push('%');
                out.push(other);
            }
        }
    }

    Ok(out)
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        "printk" | "_printk" | "printk_deferred" | "_printk_deferred" => printk,
        // dev_printk-style helpers take (dev, fmt, ...) or (level, dev, fmt, ...).
        "_dev_info" | "_dev_warn" | "_dev_err" | "_dev_notice" | "_dev_crit" | "_dev_alert" => _dev_info,
        "dev_printk" | "_dev_printk" => dev_printk,
        "__warn_printk" => __warn_printk,
        "panic" => panic,
        "dump_stack" | "__stack_chk_fail" | "__ubsan_handle_out_of_bounds" => diagnostic_stub,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn printk(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let fmt = api_try!(emu, call.arg(emu, 0));
    let line = api_try!(emu, format(emu, fmt, 1));
    let n = line.len() as u64;
    emu.kernel_log_line(line);
    call.set_return(emu, n);
}

fn _dev_info(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let fmt = api_try!(emu, call.arg(emu, 1));
    let line = api_try!(emu, format(emu, fmt, 2));
    emu.kernel_log_line(line);
    call.set_return(emu, 0);
}

fn dev_printk(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let fmt = api_try!(emu, call.arg(emu, 2));
    let line = api_try!(emu, format(emu, fmt, 3));
    emu.kernel_log_line(line);
    call.set_return(emu, 0);
}

fn __warn_printk(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let fmt = api_try!(emu, call.arg(emu, 0));
    let line = api_try!(emu, format(emu, fmt, 1));
    emu.kernel_log_line(format!("WARNING: {}", line));
    call.set_return(emu, 0);
}

fn panic(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let fmt = api_try!(emu, call.arg(emu, 0));
    let line = api_try!(emu, format(emu, fmt, 1));
    emu.kernel_log_line(format!("Kernel panic - not syncing: {}", line));
    emu.stop();
}

fn diagnostic_stub(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    emu.kernel_log_line(format!("{} called", symbol));
    call.set_return(emu, 0);
}
