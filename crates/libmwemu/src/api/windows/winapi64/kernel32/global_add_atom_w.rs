use crate::emu;
use crate::windows::constants;

pub fn GlobalAddAtomW(emu: &mut emu::Emu) {
    let lpString = emu.regs().rcx as u32;

    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalAddAtomW {}", lpString);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
