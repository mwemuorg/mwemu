use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_tolower(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let c = api_try!(emu, call.arg_u8(emu, 0));
    call.set_return(emu, (c as char).to_ascii_lowercase() as u64);
}
