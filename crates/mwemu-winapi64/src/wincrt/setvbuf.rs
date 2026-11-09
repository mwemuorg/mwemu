use crate::emu;

pub(super) fn setvbuf(emu: &mut emu::Emu) {
    // int setvbuf(FILE *stream, char *buf, int mode, size_t size): configures
    // stream buffering. We don't do real buffered I/O, so accept it and report
    // success (0).
    let stream = emu.regs().rcx;
    let mode = emu.regs().r8;
    log_red!(
        emu,
        "wincrt!setvbuf stream: 0x{:x} mode: 0x{:x}",
        stream,
        mode
    );
    emu.regs_mut().rax = 0;
}
