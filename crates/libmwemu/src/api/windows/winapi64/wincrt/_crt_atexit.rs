use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _crt_atexit(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let function = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!_crt_atexit function: 0x{:x}", function);
    call.set_return(emu, 0);
}
