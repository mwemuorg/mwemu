use crate::emu;
use crate::winapi::helper;
pub fn FreeLibrary(emu: &mut emu::Emu) {
    let hmodule = emu.regs().rcx;
    log_red!(emu, "kernel32!FreeLibrary hModule: 0x{:x}", hmodule);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
