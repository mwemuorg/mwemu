use crate::emu;
use crate::winapi::helper;
pub fn BasepReleaseSxsCreateProcessUtilityStruct(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!BasepReleaseSxsCreateProcessUtilityStruct param1:0x{:x}",
        param1
    );
    let handle = helper::handler_create("BasepReleaseSxsCreateProcessUtilityStruct");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
