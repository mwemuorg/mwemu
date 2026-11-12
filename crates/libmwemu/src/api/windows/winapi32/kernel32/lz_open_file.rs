use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn LZOpenFileA(emu: &mut emu::Emu) {
    lz_open_file(emu, "LZOpenFileA", false);
}

fn lz_open_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpReOpenBuf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _wStyle = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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
