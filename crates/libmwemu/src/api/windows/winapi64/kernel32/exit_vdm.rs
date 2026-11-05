use crate::emu;

pub fn ExitVDM(emu: &mut emu::Emu) {
    let is_wow = emu.regs().rcx;
    let exit_code = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!ExitVDM isWow:{} exitCode:{}",
        is_wow,
        exit_code
    );
    emu.set_last_error(0);
}
