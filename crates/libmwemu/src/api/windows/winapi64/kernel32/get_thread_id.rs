use crate::emu;

pub fn GetThreadId(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    for i in 0..emu.threads.len() {
        if emu.threads[i].handle == hndl {
            emu.regs_mut().rax = emu.threads[i].id;
            log_red!(
                emu,
                "kernel32!GetThreadId hndl:{} (requested handle exists and its tid {})",
                hndl,
                emu.threads[i].id
            );
            return;
        }
    }
    log_red!(
        emu,
        "kernel32!GetThreadId hndl:{} (requested handle doesn't exist, returning a fake handle for now but should return zero.)",
        hndl
    );
    emu.regs_mut().rax = 0x2c2878; // if handle not found should return zero.
}
