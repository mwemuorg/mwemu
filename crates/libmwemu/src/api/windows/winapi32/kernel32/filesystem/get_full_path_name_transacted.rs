use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetFullPathNameTransactedA(emu: &mut emu::Emu) {
    get_full_path_name_transacted(emu, "GetFullPathNameTransactedA", false);
}

fn get_full_path_name_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpFileName = WinApiAbi::arg(emu, 0) as u32;
    let _nBufferLength = WinApiAbi::arg(emu, 1) as u32;
    let _lpBuffer = WinApiAbi::arg(emu, 2) as u32;
    let _lpFilePart = WinApiAbi::arg(emu, 3) as u32;
    let _hTransaction = WinApiAbi::arg(emu, 4) as u32;

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
