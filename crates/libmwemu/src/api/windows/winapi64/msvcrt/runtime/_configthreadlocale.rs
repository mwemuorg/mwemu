use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _configthreadlocale(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let per_thread = api_try!(emu, call.arg_i32(emu, 0));
    log_red!(emu, "msvcrt!_configthreadlocale type:{}", per_thread);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, per_thread as u32);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0); // _ENABLE_PER_THREAD_LOCALE_OLD = 0
}
