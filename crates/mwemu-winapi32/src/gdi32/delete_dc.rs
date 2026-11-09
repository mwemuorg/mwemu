use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DeleteDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0) as u64;
    log_red!(emu, "gdi32!DeleteDC hdc:0x{:x}", hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
