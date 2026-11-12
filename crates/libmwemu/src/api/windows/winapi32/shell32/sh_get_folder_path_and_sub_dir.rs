use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHGetFolderPathAndSubDirA(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirA", false);
}

fn sh_get_folder_path_and_sub_dir(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let csidl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
