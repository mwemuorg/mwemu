use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn clock(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let tick_val = emu.tick as u64;
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, tick_val);
    }
    log_red!(emu, "msvcrt!clock => {}", tick_val);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, tick_val);
}
