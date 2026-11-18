use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn crt_rand(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let val = (emu
        .pos
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        >> 33)
        & 0x7fff;
    log_red!(emu, "wincrt!rand => {}", val);
    call.set_return(emu, val);
}
