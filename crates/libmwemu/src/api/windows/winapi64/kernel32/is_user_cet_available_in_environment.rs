use crate::emu;
pub fn IsUserCetAvailableInEnvironment(emu: &mut emu::Emu) {
    let user_cet_env = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!IsUserCetAvailableInEnvironment env:0x{:x} => FALSE",
        user_cet_env
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
