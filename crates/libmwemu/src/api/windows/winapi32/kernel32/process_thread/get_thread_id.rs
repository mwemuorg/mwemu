use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetThreadId(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 1);

    emu.stack_pop32(false);

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
