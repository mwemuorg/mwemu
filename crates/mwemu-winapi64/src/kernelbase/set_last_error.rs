use crate::emu;
use crate::winapi::helper;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err = emu.regs().rcx;
    log_red!(emu, "kernelbase!SetLastError err:{}", err);
    helper::advance_tick(emu, 1);
    emu.set_last_error(err);
}
