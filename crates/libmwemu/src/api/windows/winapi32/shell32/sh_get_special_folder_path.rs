use crate::emu;
use crate::winapi::common::strings;

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathA", false);
}

fn sh_get_special_folder_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let path_buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let csidl = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let create = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
