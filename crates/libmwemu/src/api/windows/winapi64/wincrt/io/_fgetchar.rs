use crate::emu;
use crate::winapi::helper;

pub fn _fgetchar(emu: &mut emu::Emu, api: &str) {
    let stream = emu.regs().rcx;
    log_red!(emu, "wincrt!{} stream:0x{:x}", api, stream);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 0x41;
}
