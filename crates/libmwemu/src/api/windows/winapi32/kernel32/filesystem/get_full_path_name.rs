use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameA", false);
}

fn get_full_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file_ptr = WinApiAbi::arg(emu, 0);
    let size = WinApiAbi::arg(emu, 1) as u32;
    let buff = WinApiAbi::arg(emu, 2) as u32;
    let path = WinApiAbi::arg(emu, 3) as u32;

    let filename = strings::read_text(emu, file_ptr, wide);

    log_red!(emu, "kernel32!{api} file: {}", filename);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 10;
}

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameW", true);
}
