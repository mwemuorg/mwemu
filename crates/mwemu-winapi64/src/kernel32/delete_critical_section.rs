use crate::emu;

pub fn DeleteCriticalSection(emu: &mut emu::Emu) {
    let cs = emu.regs().rcx;
    log_red!(emu, "kernel32!DeleteCriticalSection 0x{:x}", cs);
}
