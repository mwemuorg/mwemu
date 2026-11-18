use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _set_errno(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let err_value = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!_set_errno value:{}", err_value);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, err_value);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0); // success
}
