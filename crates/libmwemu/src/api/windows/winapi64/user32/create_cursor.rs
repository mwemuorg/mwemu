use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateCursor(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let x_hotspot = emu.regs().rdx as i32;
    let y_hotspot = emu.regs().r8 as i32;
    let width = emu.regs().r9 as i32;
    let height = WinApiAbi::arg(emu, 4) as i32;
    let and_plane = WinApiAbi::arg(emu, 5);
    let xor_plane = WinApiAbi::arg(emu, 6);
    log_red!(
        emu,
        "user32!CreateCursor hinst:0x{:x} hot:({},{}) size:{}x{}",
        hinstance,
        x_hotspot,
        y_hotspot,
        width,
        height
    );
    let hndl = helper::handler_create("user32://CreateCursor");
    emu.regs_mut().rax = hndl;
}
