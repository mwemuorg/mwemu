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
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
