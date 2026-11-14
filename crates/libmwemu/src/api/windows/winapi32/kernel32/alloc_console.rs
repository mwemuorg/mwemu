use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn AllocConsole(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    log_red!(emu, "kernel32!AllocConsole arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    let handle = helper::handler_create("allocconsole");
    emu.regs_mut().rax = handle;
}
