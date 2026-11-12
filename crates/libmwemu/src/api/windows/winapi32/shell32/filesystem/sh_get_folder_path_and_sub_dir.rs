use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHGetFolderPathAndSubDirA(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirA", false);
}

fn sh_get_folder_path_and_sub_dir(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = WinApiAbi::arg(emu, 0);
    let csidl = WinApiAbi::arg(emu, 1);
    let token = WinApiAbi::arg(emu, 2);
    let flags = WinApiAbi::arg(emu, 3);
    let path_ptr = WinApiAbi::arg(emu, 4);
    if path_ptr != 0 {
        strings::write_string(emu, path_ptr, "C:\\Users\\mwemu", wide);
    }
    log_red!(emu, "shell32!{api} csidl:0x{:x} => C:\\Users\\mwemu", csidl);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}

pub fn SHGetFolderPathAndSubDirW(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirW", true);
}
