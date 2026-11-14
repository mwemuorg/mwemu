use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateCompatibleDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!CreateCompatibleDC hdc:0x{:x}", hdc);
    let hndl = helper::handler_create("gdi32://CreateCompatibleDC");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}
