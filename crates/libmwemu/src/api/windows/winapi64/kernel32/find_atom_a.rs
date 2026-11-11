
use crate::windows::constants::errors::*;
use crate::emu;

pub fn FindAtomA(emu: &mut emu::Emu) {
    let lpString = emu.regs().rcx as u32;

    let lpString = emu.maps.read_string(lpString as u64);

    log_red!(emu, "kernel32!FindAtomA {}", lpString);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
