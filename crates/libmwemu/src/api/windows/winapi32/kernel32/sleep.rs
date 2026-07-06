use crate::emu;

pub fn Sleep(emu: &mut emu::Emu) {
    let millis = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!Sleep cannot read millis");

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
