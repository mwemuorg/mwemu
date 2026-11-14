use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetFolderPathA(emu: &mut emu::Emu) {
    sh_get_folder_path(emu, "SHGetFolderPathA", false);
}

fn sh_get_folder_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = WinApiAbi::arg(emu, 0) as u32;
    let csidl = WinApiAbi::arg(emu, 1) as u32;
    let token = WinApiAbi::arg(emu, 2) as u32;
    let flags = WinApiAbi::arg(emu, 3) as u32;
    let path_buf = WinApiAbi::arg(emu, 4);
    let path = match csidl {
        0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
        0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
        0x0024 => "C:\\Windows",
        0x0025 => "C:\\Windows\\System32",
        0x0027 => "C:\\Program Files",
        0x002b => "C:\\Program Files (x86)",
        0x0005 => "C:\\Users\\mwemu_user\\Documents",
        0x0028 => "C:\\Users\\mwemu_user\\Desktop",
        _ => "C:\\Users\\mwemu_user",
    };
    if path_buf != 0 {
        strings::write_string(emu, path_buf, path, wide);
    }
    log_red!(emu, "shell32!{api} csidl:0x{:x} => '{}'", csidl, path);
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn SHGetFolderPathW(emu: &mut emu::Emu) {
    sh_get_folder_path(emu, "SHGetFolderPathW", true);
}
