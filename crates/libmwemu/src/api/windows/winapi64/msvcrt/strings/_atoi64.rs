use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _atoi64(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = if api == "_wtoi64" {
        emu.maps.read_wide_string(str_ptr)
    } else {
        emu.maps.read_string(str_ptr)
    };
    let val = s.trim().parse::<i64>().unwrap_or(0);
    log_red!(emu, "msvcrt!{} '{}' => {}", api, s, val);
    emu.set_last_error(0);
    call.set_return(emu, val as u64);
}
