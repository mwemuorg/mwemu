use super::EOF;
use super::stream_handle;
use crate::emu;

pub(super) fn fflush(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let ok = file == 0 || stream_handle(file).is_some();
    log_red!(emu, "wincrt!fflush 0x{:x}", file);
    emu.regs_mut().rax = if ok { 0 } else { EOF };
}
