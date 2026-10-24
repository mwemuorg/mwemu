use crate::emu;
use crate::windows::constants;

pub fn CreateWaitableTimerExA(emu: &mut emu::Emu) {
    let _lpTimerAttributes = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpTimerName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _dwDesiredAccess = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpTimerName = emu.maps.read_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerExA {}", lpTimerName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
