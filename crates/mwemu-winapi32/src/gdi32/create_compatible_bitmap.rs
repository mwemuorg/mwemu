use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let w = arg(emu, 1);
    let h = arg(emu, 2);
    log_red!(emu, "gdi32!CreateCompatibleBitmap {}x{}", w, h);
    let hndl = helper::handler_create("gdi32://CreateCompatibleBitmap");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 3);
}
