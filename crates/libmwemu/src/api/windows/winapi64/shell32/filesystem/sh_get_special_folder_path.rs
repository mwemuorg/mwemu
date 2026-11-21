use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::shell32::csidl_to_path;

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathA", false);
}

fn sh_get_special_folder_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let psz_path = api_try!(emu, call.arg(emu, 1));
    let csidl = api_try!(emu, call.arg_i32(emu, 2));

    log_red!(emu, "shell32!{api} csidl: 0x{:x}", csidl);

    match csidl_to_path(csidl) {
        Some(path) => {
            if psz_path != 0 {
                strings::write_string(emu, psz_path, path, wide);
            }
            emu.set_last_error(0);
            call.set_return(emu, 1); // TRUE
        }
        None => {
            call.set_return(emu, 42); // FALSE
        }
    }
}

pub fn SHGetSpecialFolderPathW(emu: &mut emu::Emu) {
    sh_get_special_folder_path(emu, "SHGetSpecialFolderPathW", true);
}
