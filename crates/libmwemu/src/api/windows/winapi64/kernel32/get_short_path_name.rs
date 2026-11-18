use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetShortPathNameA(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameA", false);
}

fn get_short_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let long = api_try!(emu, call.arg(emu, 0));
    let short = api_try!(emu, call.arg(emu, 1));
    let buf_len = api_try!(emu, call.arg_u32(emu, 2));
    let path = strings::read_text(emu, long, wide);
    log_red!(emu, "kernel32!{api} '{}'", path);
    if short != 0 && buf_len > path.len() as u32 {
        strings::write_string(emu, short, &path, wide);
    }
    call.set_return(emu, path.len() as u64);
}

pub fn GetShortPathNameW(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameW", true);
}
