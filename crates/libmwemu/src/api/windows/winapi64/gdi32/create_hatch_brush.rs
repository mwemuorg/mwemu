use crate::emu;
use crate::winapi::helper;

pub fn CreateHatchBrush(emu: &mut emu::Emu) {
    let style = emu.regs().rcx as u32;
    let color = emu.regs().rdx as u32;
    log_red!(
        emu,
        "gdi32!CreateHatchBrush style:{} color:0x{:06x}",
        style,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}
