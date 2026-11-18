use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathCchSkipRoot(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let result_ptr = api_try!(emu, call.arg(emu, 1));
    let path = emu.maps.read_wide_string(path_ptr);
    let skip = if path.len() >= 3 && path.as_bytes()[1] == b':' && path.as_bytes()[2] == b'\\' {
        3
    } else if let Some(rest) = path.strip_prefix("\\\\") {
        if let Some(pos) = rest.find('\\') {
            pos + 3
        } else {
            path.len()
        }
    } else if path.starts_with('\\') {
        1
    } else {
        0
    };
    let after_root = path_ptr + (skip as u64) * 2;
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, after_root);
    }
    log_red!(emu, "kernelbase!PathCchSkipRoot '{}' skip:{}", path, skip);
    call.set_return(emu, 0); // S_OK
}
