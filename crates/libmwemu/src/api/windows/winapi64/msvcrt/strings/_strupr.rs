use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _strupr(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(s_ptr);
    let upper = s.to_ascii_uppercase();
    if s_ptr != 0 {
        emu.maps.write_string(s_ptr, &upper);
    }
    log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, upper);
    emu.set_last_error(0);
    call.set_return(emu, s_ptr);
}
