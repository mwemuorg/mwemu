use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RemoveDirectoryA(emu: &mut emu::Emu) {
    remove_directory(emu, "RemoveDirectoryA", false);
}

fn remove_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = WinApiAbi::arg(emu, 0);
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}'", path);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

pub fn RemoveDirectoryW(emu: &mut emu::Emu) {
    remove_directory(emu, "RemoveDirectoryW", true);
}
