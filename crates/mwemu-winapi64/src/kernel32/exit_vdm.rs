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
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
}
