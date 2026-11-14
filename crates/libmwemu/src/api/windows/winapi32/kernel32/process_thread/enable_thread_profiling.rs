use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn EnableThreadProfiling(emu: &mut emu::Emu) {
    let _ThreadHandle = WinApiAbi::arg(emu, 0) as u32;
    let _Flags = WinApiAbi::arg(emu, 1) as u32;
    let _HardwareCounters = WinApiAbi::arg(emu, 2) as u32;
    let _PerformanceDataHandle = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!EnableThreadProfiling");

    emu.regs_mut().rax = ERROR_SUCCESS;

    stdcall_ret(emu, 5);
}
