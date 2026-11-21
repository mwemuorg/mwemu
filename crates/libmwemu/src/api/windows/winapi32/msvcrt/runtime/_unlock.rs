use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _unlock(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let lock_num = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!_unlock {}", lock_num);
    helper::advance_tick(emu, 1);
}
