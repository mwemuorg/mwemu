use crate::emu;
use crate::winapi::helper;

pub fn CreateIcon(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let width = emu.regs().rdx as i32;
    let height = emu.regs().r8 as i32;
    let planes = emu.regs().r9 as u8;
    let bits_pixel = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u8;
    let and_bits = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let xor_bits = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    log_red!(
        emu,
        "user32!CreateIcon hinst:0x{:x} size:{}x{} planes:{} bpp:{}",
        hinstance,
        width,
        height,
        planes,
        bits_pixel
    );
    let hndl = helper::handler_create("user32://CreateIcon");
    emu.regs_mut().rax = hndl;
}
