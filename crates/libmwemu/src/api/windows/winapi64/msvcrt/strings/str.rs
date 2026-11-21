use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn str(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(s_ptr);
    log_red!(emu, "msvcrt!str 0x{:x} '{}'", s_ptr, s);
    call.set_return(emu, s_ptr);
}
