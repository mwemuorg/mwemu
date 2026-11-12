use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn DeregisterEventSource(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(hndl) {
        helper::handler_close(hndl);
    }
    log_red!(emu, "advapi32!DeregisterEventSource 0x{:x}", hndl);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
