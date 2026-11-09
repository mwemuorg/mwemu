use crate::emu;
use crate::winapi::helper;
pub fn SbpIsTraceEnabled(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(emu, "kernel32!SbpIsTraceEnabled param1:0x{:x}", param1);
    let handle = helper::handler_create("SbpIsTraceEnabled");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
