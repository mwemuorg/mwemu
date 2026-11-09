use crate::emu;
use crate::winapi::helper;

pub fn ReleaseCapture(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "user32!ReleaseCapture 0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
}
