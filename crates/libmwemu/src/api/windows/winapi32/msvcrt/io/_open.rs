use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _open(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = emu.maps.read_string(path_ptr);
    log_red!(emu, "msvcrt!{} '{}'", api, path);
    call.set_return(emu, helper::handler_create(&path));
}
