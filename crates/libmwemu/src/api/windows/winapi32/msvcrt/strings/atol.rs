use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn atol(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(str_ptr);
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!atol '{}' => {}", s, val);

    call.set_return(emu, val as u32 as u64);
}
