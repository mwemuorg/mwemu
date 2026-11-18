use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ispunct(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let ch = api_try!(emu, call.arg_u8(emu, 0));
    log_red!(emu, "wincrt!ispunct ch:{}", ch as char);
    helper::advance_tick(emu, 1);
    let result = if ch.is_ascii_punctuation() {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    call.set_return(emu, result);
}
