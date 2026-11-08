use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateSolidBrush(emu: &mut emu::Emu) {
    let color = arg(emu, 0);
    log_red!(emu, "gdi32!CreateSolidBrush color:0x{:06x}", color);
    let hndl = helper::handler_create("gdi32://CreateSolidBrush");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}
