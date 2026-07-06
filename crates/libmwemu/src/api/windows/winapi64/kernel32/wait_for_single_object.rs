use crate::{emu, windows::constants};

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

    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFFFFFF {
        emu.tick += millis as usize;
    }

    emu.regs_mut().rax = constants::WAIT_TIMEOUT;
}
