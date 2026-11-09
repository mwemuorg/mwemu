use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsUserCetAvailableInEnvironment(emu: &mut emu::Emu) {
    let user_cet_env = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!IsUserCetAvailableInEnvironment env:0x{:x} => FALSE",
        user_cet_env
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
