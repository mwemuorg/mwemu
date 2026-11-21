use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _configure_narrow_argv(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let mode = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!_configure_narrow_argv mode: 0x{:x}", mode);
    call.set_return(emu, 0);
}
