use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn rand(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let val = ((emu.tick as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        >> 33) as u32
        & 0x7fff;
    log_red!(emu, "msvcrt!rand => {}", val);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, val as u64);
}
