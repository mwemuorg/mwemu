use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetFullPathNameTransactedA(emu: &mut emu::Emu) {
    get_full_path_name_transacted(emu, "GetFullPathNameTransactedA", false);
}

fn get_full_path_name_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _nBufferLength = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpBuffer = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpFilePart = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn GetFullPathNameTransactedW(emu: &mut emu::Emu) {
    get_full_path_name_transacted(emu, "GetFullPathNameTransactedW", true);
}
