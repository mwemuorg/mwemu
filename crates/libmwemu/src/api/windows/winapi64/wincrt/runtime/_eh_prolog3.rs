use crate::emu;
use crate::winapi::helper;

pub fn _EH_prolog3(emu: &mut emu::Emu, api: &str) {
    let cookie = emu.regs().rcx;
    log_red!(emu, "wincrt!{} cookie:0x{:x}", api, cookie);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 0;
}
