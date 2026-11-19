use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn putc(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let ch = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "wincrt!{} ch:'{}' (0x{:x})",
        api,
        (ch & 0xff) as u8 as char,
        ch
    );
    helper::advance_tick(emu, 1);
    call.set_return(emu, ch & 0xff);
}
