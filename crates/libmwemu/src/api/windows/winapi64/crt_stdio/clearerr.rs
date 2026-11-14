use super::with_stream;
use crate::emu;

pub(super) fn clearerr(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    log_red!(emu, "wincrt!clearerr 0x{:x}", file);
    with_stream(file, |s| {
        s.eof = false;
        s.error = false;
    });
}
