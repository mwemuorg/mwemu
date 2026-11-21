use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _set_app_type(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let app_type = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "msvcrt!_set_app_type type:{}", app_type);
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, app_type);
    }
}
