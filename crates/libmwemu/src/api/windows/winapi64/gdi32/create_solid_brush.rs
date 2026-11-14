use crate::emu;
use crate::winapi::helper;

pub fn CreateSolidBrush(emu: &mut emu::Emu) {
    let color = emu.regs().rcx as u32;
    log_red!(emu, "gdi32!CreateSolidBrush color:0x{:06x}", color);
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}
