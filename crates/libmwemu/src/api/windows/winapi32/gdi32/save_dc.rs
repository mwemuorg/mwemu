use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SaveDC(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    log_red!(emu, "gdi32!SaveDC hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
