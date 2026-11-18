use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn feof(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fp = api_try!(emu, call.arg(emu, 0));
    let is_eof = helper::handler_eof(fp);
    log_red!(emu, "msvcrt!feof fp:0x{:x} => {}", fp, is_eof as u32);
    emu.set_last_error(0);
    call.set_return(emu, is_eof as u64);
}
