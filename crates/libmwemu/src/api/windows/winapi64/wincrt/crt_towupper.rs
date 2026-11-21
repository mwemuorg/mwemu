use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_towupper(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let c = api_try!(emu, call.arg_u16(emu, 0));
    let ch = char::from_u32(c as u32).unwrap_or('\0');
    let upper = ch.to_ascii_uppercase() as u64;
    call.set_return(emu, upper);
}
