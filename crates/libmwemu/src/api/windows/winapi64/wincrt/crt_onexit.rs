use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_onexit(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let func = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!_onexit 0x{:x}", func);
    call.set_return(emu, func);
}
