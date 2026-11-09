use crate::emu;
use crate::winapi::helper;

pub fn CryptGetUserKey(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "crypt32!CryptGetUserKey arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptgetuserkey");
    emu.regs_mut().rax = handle;
}
