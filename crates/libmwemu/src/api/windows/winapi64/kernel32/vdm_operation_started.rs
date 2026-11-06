
use crate::emu;
use crate::winapi::helper;
pub fn VDMOperationStarted(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(emu, "kernel32!VDMOperationStarted param1:0x{:x}", param1);
    let handle = helper::handler_create("VdmOperationStarted");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
