use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SelectObject(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let obj = arg(emu, 1);
    log_red!(emu, "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}", hdc, obj);
    let prev = helper::handler_create("gdi32://prev_object");
    assert!(prev < 0x00000001_00000000);
    emu.regs_mut().rax = prev;
    stdcall_ret(emu, 2);
}
