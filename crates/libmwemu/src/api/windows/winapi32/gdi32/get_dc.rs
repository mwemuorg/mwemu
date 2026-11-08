use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    log_red!(emu, "gdi32!GetDC hwnd:0x{:x}", hwnd);
    let hndl = helper::handler_create("gdi32://GetDC");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}
