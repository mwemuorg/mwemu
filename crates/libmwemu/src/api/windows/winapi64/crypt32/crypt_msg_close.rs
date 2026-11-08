use crate::emu;
use crate::winapi::helper;

pub fn CryptMsgClose(emu: &mut emu::Emu) {
    let msg = emu.regs().rcx;
    log_red!(emu, "crypt32!CryptMsgClose msg:0x{:x}", msg);
    if msg != 0 {
        helper::handler_close(msg);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
