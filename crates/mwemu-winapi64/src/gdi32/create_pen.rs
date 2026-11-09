use crate::emu;
use crate::winapi::helper;

pub fn CreatePen(emu: &mut emu::Emu) {
    let style = emu.regs().rcx as u32;
    let width = emu.regs().rdx as i32;
    let color = emu.regs().r8 as u32;
    let style_name = match style {
        0 => "PS_SOLID",
        1 => "PS_DASH",
        2 => "PS_DOT",
        3 => "PS_DASHDOT",
        4 => "PS_DASHDOTDOT",
        5 => "PS_NULL",
        6 => "PS_INSIDEFRAME",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!CreatePen style:{} ({}) width:{} color:0x{:06x}",
        style,
        style_name,
        width,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://pen");
}
