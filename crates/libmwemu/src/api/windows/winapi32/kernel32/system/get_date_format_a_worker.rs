use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetDateFormatAWorker(emu: &mut emu::Emu) {
    let _a1 = WinApiAbi::arg(emu, 0) as u32;
    let _Locale = WinApiAbi::arg(emu, 1) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 2) as u32;
    let _a4 = WinApiAbi::arg(emu, 3) as u32;
    let _a5 = WinApiAbi::arg(emu, 4) as u32;
    let _a6 = WinApiAbi::arg(emu, 5) as u32;
    let _cbMultiByte = WinApiAbi::arg(emu, 6) as u32;

    log_red!(emu, "kernel32!GetDateFormatAWorker");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
