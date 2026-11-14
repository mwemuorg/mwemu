use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn ConnectNamedPipe(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let overlapped = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!ConnectNamedPipe hndl: 0x{:x}", handle);
    if !helper::handler_exist(handle) {
        log::trace!("\tinvalid handle.");
    }

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
