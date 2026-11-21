use super::set_errno;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _set_errno(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    set_errno(emu, api_try!(emu, call.arg_u32(emu, 0)));
    call.set_return(emu, 0);
}
