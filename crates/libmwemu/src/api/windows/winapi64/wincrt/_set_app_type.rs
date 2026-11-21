use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _set_app_type(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let app_type = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!_set_app_type app_type: 0x{:x}", app_type);
    call.set_return(emu, 0);
}
