use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathFindFileNameA(emu: &mut emu::Emu) {
    path_find_file_name(emu, "PathFindFileNameA", false);
}

pub fn PathFindFileNameW(emu: &mut emu::Emu) {
    path_find_file_name(emu, "PathFindFileNameW", true);
}

fn path_find_file_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!{api} path: '{}'", path);
    if wide {
        if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
            emu.set_last_error(0);
            call.set_return(emu, path_ptr + ((pos as u64) + 1) * 2);
        } else {
            call.set_return(emu, path_ptr);
        }
    } else {
        if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
            emu.set_last_error(0);
            call.set_return(emu, path_ptr + (pos as u64) + 1);
        } else {
            call.set_return(emu, path_ptr);
        }
    }
}
