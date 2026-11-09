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
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
}
