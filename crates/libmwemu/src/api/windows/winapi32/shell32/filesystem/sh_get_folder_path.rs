use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SHGetFolderPathA(emu: &mut emu::Emu) {
    sh_get_folder_path(emu, "SHGetFolderPathA", false);
}

fn sh_get_folder_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg_u32(emu, 0));
    let csidl = api_try!(emu, call.arg_u32(emu, 1));
    let token = api_try!(emu, call.arg_u32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let path_buf = api_try!(emu, call.arg(emu, 4));
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
    call.cleanup_args(emu, 5);
    call.set_return(emu, 0);
}

pub fn SHGetFolderPathW(emu: &mut emu::Emu) {
    sh_get_folder_path(emu, "SHGetFolderPathW", true);
}
