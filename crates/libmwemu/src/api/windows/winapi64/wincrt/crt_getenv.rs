use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_getenv(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "wincrt!{} '{}' => NULL", api, name);
    call.set_return(emu, 0);
}
