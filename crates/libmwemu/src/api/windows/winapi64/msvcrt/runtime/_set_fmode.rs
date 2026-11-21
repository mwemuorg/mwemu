use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _set_fmode(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let mode = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!_set_fmode mode:0x{:x}", mode);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, mode);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0); // success
}
