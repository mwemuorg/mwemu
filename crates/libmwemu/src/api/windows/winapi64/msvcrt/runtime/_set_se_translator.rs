use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _set_se_translator(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let handler = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "msvcrt!{} handler:0x{:x}", api, handler);
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, handler);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
