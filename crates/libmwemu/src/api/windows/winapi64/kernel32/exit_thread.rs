use crate::emu;

pub fn ExitThread(emu: &mut emu::Emu) {
    let code = emu.regs().rcx;

    log_red!(emu, "kernel32!ExitThread code: {}", code);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.stop();
}
