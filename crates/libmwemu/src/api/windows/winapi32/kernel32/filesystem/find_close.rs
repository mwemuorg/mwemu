use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn FindClose(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0);

    log_red!(emu, "kernel32!FindClose");

    emu.stack_pop32(false);

    helper::handler_close(hndl);
    emu.regs_mut().rax = 1;
}
