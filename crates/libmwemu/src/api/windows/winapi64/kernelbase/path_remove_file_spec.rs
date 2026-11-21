use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathRemoveFileSpecA(emu: &mut emu::Emu) {
    path_remove_file_spec(emu, "PathRemoveFileSpecA", false);
}

pub fn PathRemoveFileSpecW(emu: &mut emu::Emu) {
    path_remove_file_spec(emu, "PathRemoveFileSpecW", true);
}

fn path_remove_file_spec(emu: &mut emu::Emu, api: &str, wide: bool) {
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
            emu.maps.write_word(path_ptr + (pos as u64) * 2, 0);
            call.set_return(emu, 1);
        } else {
            call.set_return(emu, 0);
        }
    } else {
        if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
            emu.maps.write_byte(path_ptr + pos as u64, 0);
            call.set_return(emu, 1); // TRUE, spec removed
        } else {
            call.set_return(emu, 0); // FALSE
        }
    }
}
