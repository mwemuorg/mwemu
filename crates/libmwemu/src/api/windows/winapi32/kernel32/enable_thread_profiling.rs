use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn EnableThreadProfiling(emu: &mut emu::Emu) {
    let _ThreadHandle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _HardwareCounters = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _PerformanceDataHandle = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!EnableThreadProfiling");

    emu.regs_mut().rax = ERROR_SUCCESS;

    stdcall_ret(emu, 5);
}
