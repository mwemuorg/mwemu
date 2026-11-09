use crate::emu;
use crate::winapi::helper;

pub fn CreatePatternBrush(emu: &mut emu::Emu) {
    let hbmp = emu.regs().rcx;
    log_red!(emu, "gdi32!CreatePatternBrush hbmp:0x{:x}", hbmp);
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}
