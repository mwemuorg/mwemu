use crate::emu;

pub fn LeaveCriticalSectionWhenCallbackReturns(emu: &mut emu::Emu) {
    let critical_section = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!LeaveCriticalSectionWhenCallbackReturns cs:0x{:x}",
        critical_section
    );
    emu.set_last_error(0);
}
