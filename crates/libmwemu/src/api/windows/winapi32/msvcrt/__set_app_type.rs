use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn __set_app_type(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let app_type = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
    emu.set_last_error(0);
}
