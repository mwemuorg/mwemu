use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _open(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = emu.maps.read_string(path_ptr);
    let hndl = helper::handler_create(&path);
    log_red!(emu, "wincrt!{} '{}' => 0x{:x}", api, path, hndl);
    call.set_return(emu, hndl);
}
