use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetFolderPathAndSubDirA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let csidl = emu.regs().rdx as i32;
    let htoken = emu.regs().r8;
    let dw_flags = emu.regs().r9;
    let psz_sub_dir = WinApiAbi::arg(emu, 4);
    let psz_path = WinApiAbi::arg(emu, 5);
    let sub_dir = if psz_sub_dir != 0 {
        emu.maps.read_string(psz_sub_dir)
    } else {
        String::new()
    };
    let path = format!("C:\\Users\\user\\AppData\\{}", sub_dir);
    if psz_path != 0 {
        emu.maps.write_string(psz_path, &path);
    }
    log_red!(
        emu,
        "shell32!SHGetFolderPathAndSubDirA csidl:0x{:x} sub:'{}' -> '{}'",
        csidl,
        sub_dir,
        path
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn SHGetFolderPathAndSubDirW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let csidl = emu.regs().rdx as i32;
    let htoken = emu.regs().r8;
    let dw_flags = emu.regs().r9;
    let psz_sub_dir = WinApiAbi::arg(emu, 4);
    let psz_path = WinApiAbi::arg(emu, 5);
    let sub_dir = if psz_sub_dir != 0 {
        emu.maps.read_wide_string(psz_sub_dir)
    } else {
        String::new()
    };
    let path = format!("C:\\Users\\user\\AppData\\{}", sub_dir);
    if psz_path != 0 {
        for (i, ch) in path.encode_utf16().enumerate() {
            emu.maps.write_word(psz_path + (i as u64) * 2, ch);
        }
        emu.maps.write_word(psz_path + (path.len() as u64) * 2, 0);
    }
    log_red!(
        emu,
        "shell32!SHGetFolderPathAndSubDirW csidl:0x{:x} sub:'{}' -> '{}'",
        csidl,
        sub_dir,
        path
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
