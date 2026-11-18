use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strtol(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let _endptr = api_try!(emu, call.arg_u32(emu, 1));
    let base = api_try!(emu, call.arg_u32(emu, 2));
    let s = emu.maps.read_string(str_ptr);
    let val = i64::from_str_radix(s.trim(), base).unwrap_or(0);
    log_red!(emu, "msvcrt!strtol '{}' base:{} => {}", s, base, val);

    call.set_return(emu, val as u64);
}
