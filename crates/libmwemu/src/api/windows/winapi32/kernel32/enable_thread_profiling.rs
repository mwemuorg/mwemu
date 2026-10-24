use crate::emu;
use crate::windows::constants;

pub fn EnableThreadProfiling(emu: &mut emu::Emu) {
    let _ThreadHandle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _HardwareCounters = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _PerformanceDataHandle = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!EnableThreadProfiling");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
