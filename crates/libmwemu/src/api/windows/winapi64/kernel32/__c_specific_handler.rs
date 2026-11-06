use crate::emu;

pub fn __C_specific_handler(emu: &mut emu::Emu) {
    let exception_record = emu.regs().rcx;
    let establisher_frame = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!__C_specific_handler excRec:0x{:x} frame:0x{:x}",
        exception_record,
        establisher_frame
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
