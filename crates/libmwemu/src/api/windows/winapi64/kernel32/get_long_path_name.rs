use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetLongPathNameA(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameA", false);
}

fn get_long_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let short = api_try!(emu, call.arg(emu, 0));
    let long = api_try!(emu, call.arg(emu, 1));
    let buf_len = api_try!(emu, call.arg_u32(emu, 2));
    let path = strings::read_text(emu, short, wide);
    log_red!(emu, "kernel32!{api} '{}'", path);
    if long != 0 && buf_len > path.len() as u32 {
        strings::write_string(emu, long, &path, wide);
    }
    call.set_return(emu, path.len() as u64);
}

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameW", true);
}
