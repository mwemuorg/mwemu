use crate::emu;
use crate::winapi::helper;

pub fn __setusermatherr(emu: &mut emu::Emu, api: &str) {
    let handler = emu.regs().rcx;
    log_red!(emu, "wincrt!{} handler:0x{:x}", api, handler);
    emu.regs_mut().rax = 0;
    helper::advance_tick(emu, 1);
}
