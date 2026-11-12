use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn Sleep(emu: &mut emu::Emu) {
    let millis = WinApiAbi::arg(emu, 0) as u32;

    log_red!(
        emu,
        "kernel32!Sleep millis: {}{}",
        millis,
        if emu.cfg.short_circuit_sleep {
            " [short-circuited]"
        } else {
            ""
        }
    );

    if emu.cfg.short_circuit_sleep {
        emu.tick += 1;
    } else {
        emu.tick += millis as usize;
    }

    emu.stack_pop32(false);
}
