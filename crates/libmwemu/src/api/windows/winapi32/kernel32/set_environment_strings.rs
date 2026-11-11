use crate::emu;
use crate::windows::constants::errors::*;

pub fn SetEnvironmentStringsA(emu: &mut emu::Emu) {
    let _NewEnvironment = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!SetEnvironmentStringsA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn SetEnvironmentStringsW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!SetEnvironmentStringsW '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
