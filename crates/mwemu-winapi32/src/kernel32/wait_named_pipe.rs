use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn WaitNamedPipeA(emu: &mut emu::Emu) {
    let lpNamedPipeName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _nTimeOut = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!WaitNamedPipeA {}", lpNamedPipeName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn WaitNamedPipeW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "kernel32!WaitNamedPipeW 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
