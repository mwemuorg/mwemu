use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wtol(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wincrt!_wtol '{}'", s);
    let val: i64 = s.trim().parse().unwrap_or(0);
    emu.set_last_error(0);
    call.set_return(emu, val as u64);
}
