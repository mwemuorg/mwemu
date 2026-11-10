//! `printk` and the kernel's `vsnprintf` family.
//!
//! The formatter is shared with `snprintf`/`scnprintf`/`kasprintf`, because in
//! the kernel they are all the same engine — and because a driver's log lines
//! are usually the fastest way to understand what path an emulated run took.

use crate::emu::Emu;

/// Render a kernel format string, pulling variadic arguments starting at
/// argument index `first_arg`.
///
/// Supports the conversions a driver realistically uses: `%d %i %u %x %X %o
/// %c %s %p<suffix> %%`, with `l`/`ll`/`z`/`h` length modifiers and
/// width/precision/flags skipped rather than honoured. Anything unrecognised is
/// emitted verbatim so a log line is never silently truncated.
pub fn format(emu: &mut Emu, fmt_addr: u64, first_arg: usize) -> String {
    let fmt = emu.maps.read_string(fmt_addr);
    // Kernel log levels are encoded as SOH + digit at the head of the string.
    let fmt = fmt.strip_prefix('\u{1}').map_or(fmt.as_str(), |rest| {
        rest.strip_prefix(|c: char| c.is_ascii_digit() || c == 'c')
            .unwrap_or(rest)
    });

    let mut out = String::new();
    let mut arg = first_arg;
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
        let mut next = |emu: &Emu| {
            let v = read_vararg(emu, arg);
            arg += 1;
            v
        };

        match conv {
            'd' | 'i' => out.push_str(&(next(emu) as i64).to_string()),
            'u' => out.push_str(&next(emu).to_string()),
            'x' => out.push_str(&format!("{:x}", next(emu))),
            'X' => out.push_str(&format!("{:X}", next(emu))),
            'o' => out.push_str(&format!("{:o}", next(emu))),
            'c' => out.push(next(emu) as u8 as char),
            's' => {
                let ptr = next(emu);
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
                out.push_str(&format!("0x{:x}", next(emu)));
            }
            other => {
                out.push('%');
                out.push(other);
            }
        }
    }

    out
}

/// Variadic argument `idx` of the current call: the first six travel in
/// registers, the rest sit above the return address on the stack.
fn read_vararg(emu: &Emu, idx: usize) -> u64 {
    let in_regs = if emu.cfg.arch.is_aarch64() { 8 } else { 6 };
    if idx < in_regs {
        return emu.kernel_arg(idx);
    }
    // `gateway_return` was already consumed, so the stack pointer points at the
    // first stacked argument.
    let sp = if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64().sp
    } else {
        emu.regs().rsp
    };
    emu.maps
        .read_qword(sp + ((idx - in_regs) as u64) * 8)
        .unwrap_or(0)
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        "printk" | "_printk" | "printk_deferred" | "_printk_deferred" => {
            let fmt = emu.kernel_arg(0);
            let line = format(emu, fmt, 1);
            let n = line.len() as u64;
            emu.kernel_log_line(line);
            emu.set_kernel_ret(n);
        }
        // dev_printk-style helpers take (dev, fmt, ...) or (level, dev, fmt, ...).
        "_dev_info" | "_dev_warn" | "_dev_err" | "_dev_notice" | "_dev_crit" | "_dev_alert" => {
            let fmt = emu.kernel_arg(1);
            let line = format(emu, fmt, 2);
            emu.kernel_log_line(line);
            emu.set_kernel_ret(0);
        }
        "dev_printk" | "_dev_printk" => {
            let fmt = emu.kernel_arg(2);
            let line = format(emu, fmt, 3);
            emu.kernel_log_line(line);
            emu.set_kernel_ret(0);
        }
        "__warn_printk" => {
            let fmt = emu.kernel_arg(0);
            let line = format(emu, fmt, 1);
            emu.kernel_log_line(format!("WARNING: {}", line));
            emu.set_kernel_ret(0);
        }
        "panic" => {
            let fmt = emu.kernel_arg(0);
            let line = format(emu, fmt, 1);
            emu.kernel_log_line(format!("Kernel panic - not syncing: {}", line));
            emu.stop();
        }
        "dump_stack" | "__stack_chk_fail" | "__ubsan_handle_out_of_bounds" => {
            emu.kernel_log_line(format!("{} called", symbol));
            emu.set_kernel_ret(0);
        }
        _ => return false,
    }
    true
}
