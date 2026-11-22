//! `printk` and the kernel's `vsnprintf` family.
//!
//! The formatter is shared with `snprintf`/`scnprintf`/`kasprintf`, because in
//! the kernel they are all the same engine — and because a driver's log lines
//! are usually the fastest way to understand what path an emulated run took.

use crate::api::abi::unix::ArgumentError;
use crate::api::abi::unix::{UnixApi, VariadicConvention};
use crate::api::dispatch_api;
use crate::api::unix::{ApiError, MemoryError, api_try, memory};
use crate::emu::Emu;
use crate::kernel::KernelOs;
use crate::kernel::{HandlerRegistry, kernel_handlers};

fn c_bytes(emu: &Emu, source: u64) -> Result<Vec<u8>, MemoryError> {
    let mut bytes = Vec::new();
    loop {
        let next = memory::byte_at(emu, source, bytes.len() as u64)?;
        if next == 0 {
            return Ok(bytes);
        }
        bytes.push(next);
    }
}

pub fn format(emu: &mut Emu, fmt: u64, first_arg: usize) -> Result<String, ArgumentError> {
    // Diagnostic log rendering historically treats missing string arguments as empty.
    let bytes = render(emu, fmt, first_arg, |emu, address| {
        Ok(emu.maps.read_string(address).into_bytes())
    })?;
    // printk log records can start with SOH followed by a log level.
    let bytes = match bytes.as_slice() {
        [1, level, rest @ ..] if level.is_ascii_digit() || *level == b'c' => rest,
        bytes => bytes,
    };
    Ok(String::from_utf8_lossy(bytes).into_owned())
}

/// Render the supported kernel conversions. Field padding and pointer extensions use simplified output.
pub(super) fn format_bytes(
    emu: &mut Emu,
    fmt_addr: u64,
    first_arg: usize,
) -> Result<Vec<u8>, ApiError> {
    render(emu, fmt_addr, first_arg, |emu, address| {
        Ok(c_bytes(emu, address)?)
    })
}

fn render<E: From<ArgumentError>>(
    emu: &Emu,
    fmt_addr: u64,
    first_arg: usize,
    read: impl Fn(&Emu, u64) -> Result<Vec<u8>, E>,
) -> Result<Vec<u8>, E> {
    let fmt = read(emu, fmt_addr)?;
    let mut out = Vec::new();
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
    let mut chars = fmt.into_iter().map(char::from).peekable();

    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c as u8);
            continue;
        }
        if chars.peek() == Some(&'%') {
            chars.next();
            out.push(b'%');
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
            'd' | 'i' => out.extend_from_slice((next(emu)? as i64).to_string().as_bytes()),
            'u' => out.extend_from_slice(next(emu)?.to_string().as_bytes()),
            'x' => out.extend_from_slice(format!("{:x}", next(emu)?).as_bytes()),
            'X' => out.extend_from_slice(format!("{:X}", next(emu)?).as_bytes()),
            'o' => out.extend_from_slice(format!("{:o}", next(emu)?).as_bytes()),
            'c' => out.push(next(emu)? as u8),
            's' => {
                let ptr = next(emu)?;
                if ptr == 0 {
                    out.extend_from_slice(b"(null)");
                } else {
                    out.extend(read(emu, ptr)?);
                }
            }
            'p' => {
                // `%pK`, `%px`, `%pS`, `%pI4`, … — the suffix selects a
                // rendering, not another argument.
                while matches!(chars.peek(), Some(c) if c.is_ascii_alphanumeric()) {
                    chars.next();
                }
                out.extend_from_slice(format!("0x{:x}", next(emu)?).as_bytes());
            }
            other => {
                out.push(b'%');
                out.push(other as u8);
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
