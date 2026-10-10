use crate::emu;

pub fn FormatMessageA(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;

    log_red!(emu, "kernel32!FormatMessageA flags: 0x{:x} (stub)", flags);

    emu.regs_mut().rax = 0;
}
