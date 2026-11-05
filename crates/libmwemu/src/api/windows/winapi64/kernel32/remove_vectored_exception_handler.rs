use crate::emu;

pub fn RemoveVectoredExceptionHandler(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!RemoveVectoredExceptionHandler handle:0x{:x}",
        handle
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
