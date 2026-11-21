use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn getenv(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let name = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}' => NULL", api, name);
    call.set_return(emu, 0);
}
