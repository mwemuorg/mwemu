use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    find_first_file(emu, "FindFirstFileA", false);
}

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    find_first_file(emu, "FindFirstFileW", true);
}

fn find_first_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file_ptr = WinApiAbi::arg(emu, 0);
    let find_data = WinApiAbi::arg(emu, 1) as u32;
    let file = strings::read_text(emu, file_ptr, wide);
    log_red!(emu, "kernel32!{api} file: {}", file);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    if wide {
        emu.regs_mut().rax = helper::handler_create(&file);
    } else {
        emu.regs_mut().rax = 1;
    }
}
