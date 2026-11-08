use crate::emu;
use crate::winapi::helper;

pub fn InternetAttemptConnect(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "wininet!InternetAttemptConnect arg1:0x{:x}", arg1);
    let handle = helper::handler_create("InternetAttemptConnect");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
