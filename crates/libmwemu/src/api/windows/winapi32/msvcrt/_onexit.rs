use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _onexit(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);

    emu.set_last_error(0);
    call.set_return(emu, fptr);
}
