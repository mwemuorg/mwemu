use crate::emu;
use crate::winapi::helper;

pub fn CryptDuplicateKey(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "crypt32!CryptDuplicateKey arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptduplicatekey");
    emu.regs_mut().rax = handle;
}
