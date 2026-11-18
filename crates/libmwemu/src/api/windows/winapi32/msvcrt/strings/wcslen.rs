use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn wcslen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_wide_string(s_ptr);
    let len = s.len();
    log_red!(emu, "msvcrt!wcslen '{}' => {}", s, len);

    call.set_return(emu, len as u64);
}
