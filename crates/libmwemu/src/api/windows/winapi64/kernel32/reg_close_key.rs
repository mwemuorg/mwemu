use crate::emu;
use crate::winapi::helper;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    helper::handler_close(hkey);
    log_red!(emu, "kernel32!RegCloseKey 0x{:x}", hkey);
    emu.regs_mut().rax = 0;
}
