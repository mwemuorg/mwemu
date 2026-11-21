use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wcslwr(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_wide_string(s_ptr);
    let lower = s.to_lowercase();
    if s_ptr != 0 {
        emu.maps.write_wide_string(s_ptr, &lower);
    }
    log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, lower);
    emu.set_last_error(0);
    call.set_return(emu, s_ptr);
}
