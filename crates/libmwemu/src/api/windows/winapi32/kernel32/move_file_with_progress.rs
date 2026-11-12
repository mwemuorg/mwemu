use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn MoveFileWithProgressA(emu: &mut emu::Emu) {
    move_file_with_progress(emu, "MoveFileWithProgressA", false);
}

fn move_file_with_progress(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 5);
    emu.regs_mut().rax = 1;
}

pub fn MoveFileWithProgressW(emu: &mut emu::Emu) {
    move_file_with_progress(emu, "MoveFileWithProgressW", true);
}
