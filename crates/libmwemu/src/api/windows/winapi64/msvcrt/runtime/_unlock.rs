use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _unlock(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let lock_num = api_try!(emu, call.arg_u32(emu, 0));
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, lock_num);
    }
    log_red!(emu, "msvcrt!_unlock {}", lock_num);
    helper::advance_tick(emu, 1);
}
