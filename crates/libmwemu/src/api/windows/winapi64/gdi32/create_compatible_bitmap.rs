use crate::emu;
use crate::winapi::helper;

pub fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let width = emu.regs().rdx;
    let height = emu.regs().r8;
    log_red!(
        emu,
        "gdi32!CreateCompatibleBitmap hdc:0x{:x} w:{} h:{}",
        hdc,
        width,
        height
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://CreateCompatibleBitmap");
}
