use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn fputs(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(s_ptr);
    log_red!(emu, "msvcrt!fputs '{}'", s);
    call.set_return(emu, 0);
}
