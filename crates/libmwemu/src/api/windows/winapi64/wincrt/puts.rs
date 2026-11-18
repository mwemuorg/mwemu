use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn puts(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s = api_try!(emu, call.arg(emu, 0));
    let msg = emu.maps.read_string(s);
    log_red!(emu, "wincrt!puts '{}'", msg);
    call.set_return(emu, 0);
}
