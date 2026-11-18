use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetLongPathNameA(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameA", false);
}

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameW", true);
}

fn get_long_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let short_ptr = api_try!(emu, call.arg(emu, 0));
    let long_ptr = api_try!(emu, call.arg(emu, 1));
    let buf_size = api_try!(emu, call.arg(emu, 2));
    let path = if short_ptr != 0 {
        strings::read_text(emu, short_ptr, wide)
    } else {
        String::new()
    };
    if wide {
        let char_count = path.encode_utf16().count() + 1;
        if long_ptr != 0 && buf_size > 0 {
            strings::write_string(emu, long_ptr, &path, wide);
        }
        log_red!(emu, "kernelbase!{api} '{}' => len:{}", path, char_count);
        emu.set_last_error(0);
        call.set_return(emu, char_count as u64);
    } else {
        if long_ptr != 0 && buf_size > 0 {
            strings::write_string(emu, long_ptr, &path, wide);
        }
        log_red!(emu, "kernelbase!{api} '{}' => len:{}", path, path.len() + 1);
        emu.set_last_error(0);
        call.set_return(emu, (path.len() + 1) as u64);
    }
}
