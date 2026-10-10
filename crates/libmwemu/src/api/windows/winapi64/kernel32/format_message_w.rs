use crate::emu;

pub fn FormatMessageW(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;

    log_red!(emu, "kernel32!FormatMessageW flags: 0x{:x} (stub)", flags);

    emu.regs_mut().rax = 0;
}
