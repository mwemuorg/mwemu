use crate::emu;
use crate::windows::constants;

pub fn CreateWaitableTimerW(emu: &mut emu::Emu) {
    let _lpTimerAttributes = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _bManualReset = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let lpTimerName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpTimerName = emu.maps.read_wide_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerW {}", lpTimerName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
