use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn NtVdm64CreateProcessInternalW(emu: &mut emu::Emu) {
    let _a1 = WinApiAbi::arg(emu, 0) as u32;
    let _a2 = WinApiAbi::arg(emu, 1) as u32;
    let _cchDest = WinApiAbi::arg(emu, 2) as u32;
    let _a4 = WinApiAbi::arg(emu, 3) as u32;
    let _a5 = WinApiAbi::arg(emu, 4) as u32;
    let _a6 = WinApiAbi::arg(emu, 5) as u32;
    let _a7 = WinApiAbi::arg(emu, 6) as u32;
    let _a8 = WinApiAbi::arg(emu, 7) as u32;
    let _a9 = WinApiAbi::arg(emu, 8) as u32;
    let _a10 = WinApiAbi::arg(emu, 9) as u32;
    let _a11 = WinApiAbi::arg(emu, 10) as u32;
    let _a12 = WinApiAbi::arg(emu, 11) as u32;

    log_red!(emu, "kernel32!NtVdm64CreateProcessInternalW");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..12 {
        emu.stack_pop32(false);
    }
}
