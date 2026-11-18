use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_strrchr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1));
    let str_val = emu.maps.read_string(s);
    let result = match str_val.rfind(c as char) {
        Some(pos) => s + pos as u64,
        None => 0,
    };
    call.set_return(emu, result);
}
