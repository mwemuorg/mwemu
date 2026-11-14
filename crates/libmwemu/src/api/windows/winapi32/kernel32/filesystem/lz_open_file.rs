use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn LZOpenFileA(emu: &mut emu::Emu) {
    lz_open_file(emu, "LZOpenFileA", false);
}

fn lz_open_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpFileName = WinApiAbi::arg(emu, 0) as u32;
    let _lpReOpenBuf = WinApiAbi::arg(emu, 1) as u32;
    let _wStyle = WinApiAbi::arg(emu, 2) as u32;

    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn LZOpenFileW(emu: &mut emu::Emu) {
    lz_open_file(emu, "LZOpenFileW", true);
}
