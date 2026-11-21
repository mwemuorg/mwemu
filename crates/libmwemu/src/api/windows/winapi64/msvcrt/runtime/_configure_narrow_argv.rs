use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _configure_narrow_argv(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let mode = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "msvcrt!_configure_narrow_argv mode:{}", mode);
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, mode);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0); // success
}
