#[macro_export]
macro_rules! popn {
    ($emu:expr, $n:expr) => {
        for _ in 0..$n {
            $emu.stack_pop(false);
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
        if $emu.cfg.verbose >= 1 {
            let tid = $emu.threads[$emu.current_thread_id].id;
            if $emu.cfg.nocolors {
                log::trace!(
                    "** {:x}> {}:{:x} {}",
                    tid,
                    $emu.pos,
                    $emu.regs().rip,
                    format!($($arg)*)
                );
            } else {
                log::trace!(
                    "{}** {:x}> {}:{:x} {}{}",
                    $emu.colors.light_red,
                    tid,
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
        let tid = $emu.threads[$emu.current_thread_id].id;
        if $emu.cfg.nocolors {
            log::trace!(
                "** {:x}> {}:{:x} {}",
                tid,
                $emu.pos,
                $emu.regs().rip,
                format!($($arg)*)
            );
        } else {
            log::trace!(
                "{}** {:x}> {}:{:x} {}{}",
                $emu.colors.orange,
                tid,
                $emu.pos,
                $emu.regs().rip,
                format!($($arg)*),
                $emu.colors.nc
            );
        }
    };
}

/// Propagate an unavailable operand through an instruction handler's bool result.
#[macro_export]
macro_rules! try_operand {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => return false,
        }
    };
}
