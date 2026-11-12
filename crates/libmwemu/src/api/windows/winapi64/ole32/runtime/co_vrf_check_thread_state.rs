use crate::emu;
use crate::winapi::helper;

pub fn CoVrfCheckThreadState(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    log_red!(emu, "ole32!CoVrfCheckThreadState flags:0x{:x}", flags);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
