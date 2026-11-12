use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi64::shell32::csidl_to_path;

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathA", false);
}

fn sh_get_special_folder_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let psz_path = emu.regs().rdx;
    let csidl = emu.regs().r8 as i32;

    log_red!(emu, "shell32!{api} csidl: 0x{:x}", csidl);

    match csidl_to_path(csidl) {
        Some(path) => {
            if psz_path != 0 {
                strings::write_string(emu, psz_path, path, wide);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // TRUE
        }
        None => {
            emu.regs_mut().rax = 42; // FALSE
        }
    }
}

pub fn SHGetSpecialFolderPathW(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathW", true);
}
