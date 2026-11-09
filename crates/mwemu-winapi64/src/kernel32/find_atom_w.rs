
use crate::windows::constants::errors::*;
use crate::emu;

pub fn FindAtomW(emu: &mut emu::Emu) {
    let lpString = emu.regs().rcx as u32;

    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(emu, "kernel32!FindAtomW {}", lpString);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
