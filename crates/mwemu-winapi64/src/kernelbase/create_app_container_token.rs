use crate::emu;
use crate::winapi::helper;

pub fn CreateAppContainerToken(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateAppContainerToken");
    log_red!(emu, "kernelbase!CreateAppContainerToken => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
