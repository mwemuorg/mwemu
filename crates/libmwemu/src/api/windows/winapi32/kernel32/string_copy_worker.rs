use crate::emu;
use crate::windows::constants::errors::*;

pub fn StringCopyWorkerA(emu: &mut emu::Emu) {
    string_copy_worker(emu, "StringCopyWorkerA");
}

fn string_copy_worker(emu: &mut emu::Emu, api: &str) {
    let _pszDest = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _cchDest = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _pcchNewDestLength = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _pszSrc = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _cchToCopy = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn StringCopyWorkerW(emu: &mut emu::Emu) {
    string_copy_worker(emu, "StringCopyWorkerW");
}
