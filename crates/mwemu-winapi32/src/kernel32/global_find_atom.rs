use crate::emu;
use crate::windows::constants::errors::*;

pub fn GlobalFindAtomA(emu: &mut emu::Emu) {
    let lpString = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let lpString = emu.maps.read_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalFindAtomA {}", lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn GlobalFindAtomW(emu: &mut emu::Emu) {
    let lpString = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalFindAtomW {}", lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
