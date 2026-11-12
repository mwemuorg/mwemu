use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::process::*;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let millis = WinApiAbi::arg(emu, 1) as u32;

    log_red!(
        emu,
        "kernel32!WaitForSingleObject  hndl: {} millis: {}{}",
        handle,
        millis,
        if emu.cfg.short_circuit_sleep {
            " [short-circuited]"
        } else {
            ""
        }
    );

    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFFFFFF {
        emu.tick += millis as usize;
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = WAIT_TIMEOUT;
}
