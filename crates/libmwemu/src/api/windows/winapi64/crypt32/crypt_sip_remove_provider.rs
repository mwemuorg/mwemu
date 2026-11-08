use crate::emu;
use crate::winapi::helper;

pub fn CryptSIPRemoveProvider(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "crypt32!CryptSIPRemoveProvider arg1:0x{:x}", arg1);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
