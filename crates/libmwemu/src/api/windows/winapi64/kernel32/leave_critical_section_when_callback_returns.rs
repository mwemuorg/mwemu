
use crate::emu;
use crate::winapi::helper;
pub fn LeaveCriticalSectionWhenCallbackReturns(emu: &mut emu::Emu) {
    let critical_section = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!LeaveCriticalSectionWhenCallbackReturns cs:0x{:x}",
        critical_section
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
}
