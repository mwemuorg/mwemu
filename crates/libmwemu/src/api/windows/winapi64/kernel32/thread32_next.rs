use crate::{emu, windows::constants};

pub fn Thread32Next(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let entry = emu.regs().rdx;

    log_red!(emu, "kernel32!Thread32Next");
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}
