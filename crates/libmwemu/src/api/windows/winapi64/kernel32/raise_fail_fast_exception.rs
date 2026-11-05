use crate::emu;

pub fn RaiseFailFastException(emu: &mut emu::Emu) {
    let exception_record = emu.regs().rcx;
    let context_record = emu.regs().rdx;
    let flags = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!RaiseFailFastException pExcept:0x{:x} pCtx:0x{:x} flags:0x{:x}",
        exception_record,
        context_record,
        flags
    );
    emu.set_last_error(0);
}
