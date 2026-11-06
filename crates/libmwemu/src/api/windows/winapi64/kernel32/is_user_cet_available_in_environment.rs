
use crate::emu;
use crate::winapi::helper;
pub fn IsUserCetAvailableInEnvironment(emu: &mut emu::Emu) {
    let user_cet_env = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!IsUserCetAvailableInEnvironment env:0x{:x} => FALSE",
        user_cet_env
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
