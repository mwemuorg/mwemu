use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathFindExtensionA(emu: &mut emu::Emu) {
    path_find_extension(emu, "PathFindExtensionA", false);
}

pub fn PathFindExtensionW(emu: &mut emu::Emu) {
    path_find_extension(emu, "PathFindExtensionW", true);
}

fn path_find_extension(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let filename = path
        .rfind('\\')
        .or_else(|| path.rfind('/'))
        .map(|p| &path[p + 1..])
        .unwrap_or(&path);
    if wide {
        if let Some(dot_pos) = filename.rfind('.') {
            let offset_in_filename =
                filename[..dot_pos].encode_utf16().count() + path.len() - filename.len();
            let ext_offset_chars = path[..path.len() - filename.len()].encode_utf16().count()
                + filename[..dot_pos].encode_utf16().count();
            emu.set_last_error(0);
            call.set_return(emu, path_ptr + (ext_offset_chars as u64) * 2);
        } else {
            let total_chars: u64 = path.encode_utf16().count() as u64;
            call.set_return(emu, path_ptr + total_chars * 2);
        }
    } else {
        if let Some(dot_pos) = filename.rfind('.') {
            let abs_dot = path.len() - filename.len() + dot_pos;
            emu.set_last_error(0);
            call.set_return(emu, path_ptr + abs_dot as u64);
        } else {
            call.set_return(emu, path_ptr + path.len() as u64);
        }
    }

    log_red!(emu, "kernelbase!{api} '{}'", path);
}
