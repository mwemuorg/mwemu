use crate::emu;
use crate::winapi::helper;

pub fn CloseDesktop(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(emu, "user32!CloseDesktop handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
}
