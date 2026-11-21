use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn freopen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = emu.maps.read_string(path_ptr);
    let hndl = helper::handler_create(&path);
    log_red!(emu, "msvcrt!freopen '{}' => 0x{:x}", path, hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
