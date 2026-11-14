use crate::emu;
use crate::winapi::helper;

pub fn OpenOrCreateStream(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("OpenOrCreateStream");
    log_red!(emu, "ole32!OpenOrCreateStream => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
