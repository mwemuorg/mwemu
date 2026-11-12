use crate::emu;
use crate::winapi::helper;

pub fn _EH_prolog(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "wincrt!{} 0x{:x}", api, arg1);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 0;
}
