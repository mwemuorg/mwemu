#[macro_export]
macro_rules! popn {
    ($emu:expr, $n:expr) => {
        for _ in 0..$n {
            $emu.stack_pop(false);
        }
    };
}

#[macro_export]
macro_rules! get_bit {
    ($val:expr, $count:expr) => {
        ($val & (1 << $count)) >> $count
    };
}

#[macro_export]
macro_rules! set_bit {
    ($val:expr, $count:expr, $bit:expr) => {
        if $bit == 1 {
            $val |= 1 << $count;
        } else {
            $val &= !(1 << $count);
        }
    };
}

#[macro_export]
macro_rules! to32 {
    ($val:expr) => {
        ($val & 0xffffffff) as u32
    };
}

#[macro_export]
macro_rules! log_red {
    ($emu:expr, $($arg:tt)*) => {
        // API-level traces (winapi32/winapi64 hooks, winsock helpers, etc.)
        // are gated by `cfg.verbose >= 1`. Under `--ssdt` we want the console
        // to show only **syscalls** by default — the API surface runs as real
        // PE bytes from the loaded DLL, and surfacing every Rust-side stub
        // call confuses the user about whether the gateway dispatcher fired
        // or the native code ran. Operators that want the full call log can
        // still pass `-v` (or higher).
        if $emu.cfg.verbose >= 1 {
            if $emu.cfg.nocolors {
                log::trace!(
                    "** {}:{:x} {}",
                    $emu.pos,
                    $emu.regs().rip,
                    format!($($arg)*)
                );
            } else {
                log::trace!(
                    "{}** {}:{:x} {}{}",
                    $emu.colors.light_red,
                    $emu.pos,
                    $emu.regs().rip,
                    format!($($arg)*),
                    $emu.colors.nc
                );
            }
        }
    };
}

#[macro_export]
macro_rules! log_orange {
    ($emu:expr, $($arg:tt)*) => {
        if $emu.cfg.nocolors {
            log::trace!(
                "** {}:{:x} {}",
                $emu.pos,
                $emu.regs().rip,
                format!($($arg)*)
            );
        } else {
            log::trace!(
                "{}** {}:{:x} {}{}",
                $emu.colors.orange,
                $emu.pos,
                $emu.regs().rip,
                format!($($arg)*),
                $emu.colors.nc
            );
        }
    };
}
