use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn fflush(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg_u32(emu, 0));

    let filename = helper::handler_get_uri(file as u64);

    log_red!(emu, "msvcrt!fflush `{}`", filename);

    call.set_return(emu, 1);
}
