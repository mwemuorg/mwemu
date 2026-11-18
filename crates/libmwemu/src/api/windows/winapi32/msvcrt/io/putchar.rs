use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn putchar(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let c = api_try!(emu, call.arg_u32(emu, 0));
    let ch_str = format!("{}", c as u8 as char);
    log_red!(emu, "msvcrt!{} '{}'", api, ch_str);
    emu.set_last_error(0);
    call.set_return(emu, c as u64);
}
