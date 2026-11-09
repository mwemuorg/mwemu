use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetDCBrushColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(emu, "gdi32!SetDCBrushColor color:0x{:06x}", color);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous color
    stdcall_ret(emu, 2);
}
