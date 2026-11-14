use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn StringVPrintfWorkerW_0(emu: &mut emu::Emu) {
    let _pszDest = WinApiAbi::arg(emu, 0) as u32;
    let _cchDest = WinApiAbi::arg(emu, 1) as u32;
    let _pcchNewDestLength = WinApiAbi::arg(emu, 2) as u32;
    let _pszFormat = WinApiAbi::arg(emu, 3) as u32;
    let _argList = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!StringVPrintfWorkerW_0");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
