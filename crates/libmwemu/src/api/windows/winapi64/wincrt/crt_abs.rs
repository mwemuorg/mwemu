use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_abs(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let val = api_try!(emu, call.arg_i64(emu, 0));
    call.set_return(emu, val.unsigned_abs());
}
