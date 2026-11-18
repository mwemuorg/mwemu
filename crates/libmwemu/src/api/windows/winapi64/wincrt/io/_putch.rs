use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _putch(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let ch = api_try!(emu, call.arg_u8(emu, 0));
    log_red!(emu, "wincrt!_putch ch:{}", ch as char);
    helper::advance_tick(emu, 1);
    call.set_return(emu, ch as u64);
}
