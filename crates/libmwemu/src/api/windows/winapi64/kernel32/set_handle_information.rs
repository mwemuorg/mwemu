use crate::emu;

pub fn SetHandleInformation(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let mask = emu.regs().rdx;
    let flags = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!SetHandleInformation handle:0x{:x} mask:0x{:x} flags:0x{:x}",
        handle,
        mask,
        flags
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
