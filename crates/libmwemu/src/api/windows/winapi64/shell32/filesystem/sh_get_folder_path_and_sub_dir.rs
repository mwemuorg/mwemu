use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetFolderPathAndSubDirA(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirA", false);
}

pub fn SHGetFolderPathAndSubDirW(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirW", true);
}

fn sh_get_folder_path_and_sub_dir(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let csidl = emu.regs().rdx as i32;
    let htoken = emu.regs().r8;
    let dw_flags = emu.regs().r9;
    let psz_sub_dir = WinApiAbi::arg(emu, 4);
    let psz_path = WinApiAbi::arg(emu, 5);
    let sub_dir = if psz_sub_dir != 0 {
        strings::read_text(emu, psz_sub_dir, wide)
    } else {
        String::new()
    };
    let path = format!("C:\\Users\\user\\AppData\\{}", sub_dir);
    if wide {
        if psz_path != 0 {
            for (i, ch) in path.encode_utf16().enumerate() {
                emu.maps.write_word(psz_path + (i as u64) * 2, ch);
            }
            emu.maps.write_word(psz_path + (path.len() as u64) * 2, 0);
        }
    } else {
        if psz_path != 0 {
            strings::write_string(emu, psz_path, &path, wide);
        }
    }

    log_red!(
        emu,
        "shell32!{api} csidl:0x{:x} sub:'{}' -> '{}'",
        csidl,
        sub_dir,
        path
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
