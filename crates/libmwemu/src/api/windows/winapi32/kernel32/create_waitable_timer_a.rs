use crate::emu;
use crate::windows::constants;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu) {
    let _lpTimerAttributes = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _bManualReset = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpTimerName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpTimerName = emu.maps.read_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerA {}", lpTimerName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
