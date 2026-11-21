use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strtol(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let _endptr = api_try!(emu, call.arg(emu, 1));
    let base = api_try!(emu, call.arg_i32(emu, 2));
    let s = emu.maps.read_string(str_ptr);
    let val = i64::from_str_radix(s.trim(), base as u32).unwrap_or(0) as u32;
    log_red!(emu, "msvcrt!strtol '{}' base:{} => {}", s, base, val);
    emu.set_last_error(0);
    call.set_return(emu, val as u64);
}
