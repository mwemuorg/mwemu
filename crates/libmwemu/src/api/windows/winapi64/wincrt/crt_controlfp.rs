use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_controlfp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let new_val = api_try!(emu, call.arg(emu, 0));
    let mask = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wincrt!_controlfp 0x{:x} 0x{:x}", new_val, mask);
    call.set_return(emu, 0x0008_001f);
}
