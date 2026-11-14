use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn FindNextFileA(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileA");
}

fn find_next_file(emu: &mut emu::Emu, api: &str) {
    let hndl = WinApiAbi::arg(emu, 0) as u32;
    let find_data = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = ERROR_NO_MORE_FILES;
}

pub fn FindNextFileW(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileW");
}
