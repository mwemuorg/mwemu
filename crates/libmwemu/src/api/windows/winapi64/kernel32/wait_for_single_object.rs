use crate::emu;
use crate::windows::constants::process::*;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let millis = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!WaitForSingleObject  hndl: {} millis: {}{}",
        hndl,
        millis,
        if emu.cfg.short_circuit_sleep {
            " [short-circuited]"
        } else {
            ""
        }
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFFFFFF {
        emu.tick += millis as usize;
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = WAIT_TIMEOUT;
}
