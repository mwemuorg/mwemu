use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _putenv(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}'", api, s);
    call.set_return(emu, 0);
}
