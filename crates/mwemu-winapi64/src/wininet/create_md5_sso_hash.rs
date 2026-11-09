use crate::emu;
use crate::winapi::helper;

pub fn CreateMD5SSOHash(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateMD5SSOHash");
    log_red!(emu, "wininet!CreateMD5SSOHash => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
