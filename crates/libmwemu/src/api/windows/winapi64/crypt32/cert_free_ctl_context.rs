use crate::emu;
use crate::winapi::helper;

pub fn CertFreeCTLContext(emu: &mut emu::Emu) {
    let context = emu.regs().rcx;
    log_red!(emu, "crypt32!CertFreeCTLContext ctx:0x{:x}", context);
    if context != 0 {
        helper::handler_close(context);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
