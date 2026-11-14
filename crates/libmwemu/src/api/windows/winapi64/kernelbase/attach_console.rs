use crate::emu;
use crate::winapi::helper;

pub fn AttachConsole(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernelbase!AttachConsole");
    let handle = helper::handler_create("AttachConsole");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
