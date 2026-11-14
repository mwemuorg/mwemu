use crate::emu;
use crate::winapi::helper;

pub fn DestroyReasons(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(emu, "user32!DestroyReasons handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
}
