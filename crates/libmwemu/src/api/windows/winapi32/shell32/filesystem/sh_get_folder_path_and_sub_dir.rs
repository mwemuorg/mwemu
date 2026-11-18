use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SHGetFolderPathAndSubDirA(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirA", false);
}

fn sh_get_folder_path_and_sub_dir(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let csidl = api_try!(emu, call.arg(emu, 1));
    let token = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    let path_ptr = api_try!(emu, call.arg(emu, 4));
    if path_ptr != 0 {
        strings::write_string(emu, path_ptr, "C:\\Users\\mwemu", wide);
    }
    log_red!(emu, "shell32!{api} csidl:0x{:x} => C:\\Users\\mwemu", csidl);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 6);
}

pub fn SHGetFolderPathAndSubDirW(emu: &mut emu::Emu) {
    sh_get_folder_path_and_sub_dir(emu, "SHGetFolderPathAndSubDirW", true);
}
