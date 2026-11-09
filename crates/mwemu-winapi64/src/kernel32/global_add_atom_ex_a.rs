
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GlobalAddAtomExA(emu: &mut emu::Emu) {
    let lpString = emu.regs().rcx as u32;
    let _Flags = emu.regs().rdx as u32;

    let lpString = emu.maps.read_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalAddAtomExA {}", lpString);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
