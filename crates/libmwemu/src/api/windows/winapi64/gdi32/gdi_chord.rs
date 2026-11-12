use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GdiChord(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = WinApiAbi::arg(emu, 4) as i32;
    let x_start = WinApiAbi::arg(emu, 5) as i32;
    let y_start = WinApiAbi::arg(emu, 6) as i32;
    let x_end = WinApiAbi::arg(emu, 7) as i32;
    let y_end = WinApiAbi::arg(emu, 8) as i32;
    log_red!(
        emu,
        "gdi32!Chord hdc:0x{:x} rect:({},{},{},{}) start:({},{}) end:({},{})",
        hdc,
        left,
        top,
        right,
        bottom,
        x_start,
        y_start,
        x_end,
        y_end
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
