use crate::emu;
use crate::windows::constants::errors::*;

pub fn SetDefaultCommConfigA(emu: &mut emu::Emu) {
    let lpszName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpCC = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpszName = emu.maps.read_string(lpszName as u64);

    log_red!(emu, "kernel32!SetDefaultCommConfigA {}", lpszName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn SetDefaultCommConfigW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!SetDefaultCommConfigW '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
