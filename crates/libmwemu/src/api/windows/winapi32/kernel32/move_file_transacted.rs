use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn MoveFileTransactedA(emu: &mut emu::Emu) {
    move_file_transacted(emu, "MoveFileTransactedA", false);
}

fn move_file_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpExistingFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpNewFileName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpProgressRoutine = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpData = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let lpExistingFileName = strings::read_text(emu, lpExistingFileName as u64, wide);
    let lpNewFileName = strings::read_text(emu, lpNewFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn MoveFileTransactedW(emu: &mut emu::Emu) {
    move_file_transacted(emu, "MoveFileTransactedW", true);
}
