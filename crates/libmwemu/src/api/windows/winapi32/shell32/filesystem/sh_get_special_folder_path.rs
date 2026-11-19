use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathA", false);
}

fn sh_get_special_folder_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg_u32(emu, 0));
    let path_buf = api_try!(emu, call.arg(emu, 1));
    let csidl = api_try!(emu, call.arg_u32(emu, 2));
    let create = api_try!(emu, call.arg_u32(emu, 3));
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
    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}

pub fn SHGetSpecialFolderPathW(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathW", true);
}
