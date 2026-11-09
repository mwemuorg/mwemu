use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminRemoveCatalog(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(emu, "crypt32!CryptCATAdminRemoveCatalog arg1:0x{:x}", arg1);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
