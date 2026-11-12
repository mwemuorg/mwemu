use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GdiEllipse(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = WinApiAbi::arg(emu, 4) as i32;
    log_red!(
        emu,
        "gdi32!Ellipse hdc:0x{:x} left:{} top:{} right:{} bottom:{}",
        hdc,
        left,
        top,
        right,
        bottom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
