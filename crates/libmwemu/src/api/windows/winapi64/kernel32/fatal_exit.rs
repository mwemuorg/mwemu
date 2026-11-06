use crate::emu;

pub fn FatalExit(emu: &mut emu::Emu) {
    let exit_code = emu.regs().rcx;
    log_red!(emu, "kernel32!FatalExit exitCode:{}", exit_code);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.running = false;
    emu.set_last_error(0);
}
