use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CopyFileTransactedA(emu: &mut emu::Emu) {
    copy_file_transacted(emu, "CopyFileTransactedA", false);
}

fn copy_file_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpExistingFileName = WinApiAbi::arg(emu, 0) as u32;
    let lpNewFileName = WinApiAbi::arg(emu, 1) as u32;
    let _lpProgressRoutine = WinApiAbi::arg(emu, 2) as u32;
    let _lpData = WinApiAbi::arg(emu, 3) as u32;
    let _pbCancel = WinApiAbi::arg(emu, 4) as u32;
    let _dwCopyFlags = WinApiAbi::arg(emu, 5) as u32;
    let _hTransaction = WinApiAbi::arg(emu, 6) as u32;

    let lpExistingFileName = strings::read_text(emu, lpExistingFileName as u64, wide);
    let lpNewFileName = strings::read_text(emu, lpNewFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}

pub fn CopyFileTransactedW(emu: &mut emu::Emu) {
    copy_file_transacted(emu, "CopyFileTransactedW", true);
}
