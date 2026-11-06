use crate::emu;
use crate::winapi::helper;
pub fn RegCloseKey(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    log_red!(emu, "kernel32!RegCloseKey hKey:0x{:x}", hkey);
    if helper::handler_exist(hkey) {
        helper::handler_close(hkey);
    }
    emu.regs_mut().rax = 0;
}
