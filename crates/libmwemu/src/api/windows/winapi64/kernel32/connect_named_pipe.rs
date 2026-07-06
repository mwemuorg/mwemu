use crate::emu;
use crate::winapi::helper;

pub fn ConnectNamedPipe(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let overlapped = emu.regs().rdx;

    log_red!(emu, "kernel32!ConnectNamedPipe hndl: 0x{:x}", handle);

    if !helper::handler_exist(handle) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1;
}
