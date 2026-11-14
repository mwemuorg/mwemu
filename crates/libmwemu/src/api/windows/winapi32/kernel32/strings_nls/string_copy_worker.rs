use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn StringCopyWorkerA(emu: &mut emu::Emu) {
    string_copy_worker(emu, "StringCopyWorkerA");
}

fn string_copy_worker(emu: &mut emu::Emu, api: &str) {
    let _pszDest = WinApiAbi::arg(emu, 0) as u32;
    let _cchDest = WinApiAbi::arg(emu, 1) as u32;
    let _pcchNewDestLength = WinApiAbi::arg(emu, 2) as u32;
    let _pszSrc = WinApiAbi::arg(emu, 3) as u32;
    let _cchToCopy = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn StringCopyWorkerW(emu: &mut emu::Emu) {
    string_copy_worker(emu, "StringCopyWorkerW");
}
