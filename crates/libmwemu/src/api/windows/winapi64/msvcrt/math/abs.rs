use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn abs(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let val = api_try!(emu, call.arg_i32(emu, 0));
    let result = val.unsigned_abs();
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, result);
    }
    log_red!(emu, "msvcrt!{} {} => {}", api, val, result);
    emu.set_last_error(0);
    call.set_return(emu, result as u64);
}
