use crate::emu;
use crate::winapi::helper;

pub fn CreateRectRgn(emu: &mut emu::Emu) {
    let x1 = emu.regs().rcx as i32;
    let y1 = emu.regs().rdx as i32;
    let x2 = emu.regs().r8 as i32;
    let y2 = emu.regs().r9 as i32;
    log_red!(
        emu,
        "gdi32!CreateRectRgn x1:{} y1:{} x2:{} y2:{}",
        x1,
        y1,
        x2,
        y2
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://region");
}
