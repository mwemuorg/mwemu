use crate::emu;
use crate::winapi::helper;

pub fn CoEnableCallCancellation(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "ole32!CoEnableCallCancellation arg1:0x{:x}", arg1);
    let handle = helper::handler_create("CoEnableCallCancellation");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1 {}
}
