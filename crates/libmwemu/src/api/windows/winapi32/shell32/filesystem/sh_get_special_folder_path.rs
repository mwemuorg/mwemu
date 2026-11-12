use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathA", false);
}

fn sh_get_special_folder_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = WinApiAbi::arg(emu, 0) as u32;
    let path_buf = WinApiAbi::arg(emu, 1);
    let csidl = WinApiAbi::arg(emu, 2) as u32;
    let create = WinApiAbi::arg(emu, 3) as u32;
    let path = match csidl {
        0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
        0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
        0x0024 => "C:\\Windows",
        0x0025 => "C:\\Windows\\System32",
        _ => "C:\\Users\\mwemu_user",
    };
    if path_buf != 0 {
        strings::write_string(emu, path_buf, path, wide);
    }
    log_red!(emu, "shell32!{api} csidl:0x{:x} => '{}'", csidl, path);
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn SHGetSpecialFolderPathW(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathW", true);
}
