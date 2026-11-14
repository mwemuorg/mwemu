use crate::emu;

pub fn WPUCompleteOverlappedRequest(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let overlapped_ptr = emu.regs().rdx;
    let error = emu.regs().r8;
    let bytes = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WPUCompleteOverlappedRequest sock:0x{:x} err:{} bytes:{}",
        sock,
        error,
        bytes
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
