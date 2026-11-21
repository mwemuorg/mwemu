use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _EH_prolog3(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let cookie = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!{} cookie:0x{:x}", api, cookie);
    emu.stack_push32(emu.regs().get_ebp() as u32);
}
