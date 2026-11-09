use crate::emu;
use crate::winapi::helper;

pub fn GetTapeStatus(emu: &mut emu::Emu) {
    let h_device = emu.regs().rcx;
    log_red!(emu, "kernel32!GetTapeStatus h:0x{:x} => NO_ERROR", h_device);
    let _ = helper::handler_exist(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
