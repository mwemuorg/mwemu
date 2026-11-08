use crate::emu;
use crate::windows::constants;

pub fn CreateSemaphoreExA(emu: &mut emu::Emu) {
    let _lpSemaphoreAttributes = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lInitialCount = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lMaximumCount = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpName = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _dwDesiredAccess = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateSemaphoreExA {}", lpName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
