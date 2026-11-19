use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wtoi(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let ws = emu.maps.read_wide_string(ptr);
    let val = ws.parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!_wtoi '{}' => {}", ws, val);
    call.set_return(emu, val as u64);
}
