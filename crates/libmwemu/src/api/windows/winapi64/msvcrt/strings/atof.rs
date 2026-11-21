use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn atof(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(str_ptr);
    let val = s.trim().parse::<f64>().unwrap_or(0.0);
    log_red!(emu, "msvcrt!{} '{}' => {}", api, s, val);
    emu.fpu_mut().set_st(0, val);
    emu.set_last_error(0);
    call.set_return(emu, val.to_bits());
}
