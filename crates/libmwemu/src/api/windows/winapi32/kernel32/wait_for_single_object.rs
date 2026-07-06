use crate::emu;
use crate::windows::constants;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!WaitForSingleObject error reading handle") as u64;
    let millis = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!WaitForSingleObject error reading millis");

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
    emu.regs_mut().rax = constants::WAIT_TIMEOUT;
}
