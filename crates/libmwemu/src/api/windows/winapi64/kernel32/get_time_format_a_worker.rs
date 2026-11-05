use crate::emu;

pub fn GetTimeFormatAWorker(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let time_ptr = emu.regs().r8;
    let format_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetTimeFormatAWorker locale:0x{:x} flags:0x{:x}",
        locale,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
