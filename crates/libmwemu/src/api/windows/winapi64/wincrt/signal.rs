use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn signal(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let sig = api_try!(emu, call.arg(emu, 0));
    let handler = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wincrt!signal sig:{} handler:0x{:x}", sig, handler);
    emu.set_last_error(0);
    call.set_return(emu, 0); // SIG_DFL
}
