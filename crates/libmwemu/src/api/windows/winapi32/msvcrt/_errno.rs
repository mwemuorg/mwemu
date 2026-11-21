use super::allocate_memory;
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn _errno(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let addr = allocate_memory(emu, 4).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_dword(addr, 0);
    }
    log_red!(emu, "msvcrt!_errno => 0x{:x}", addr);
    call.set_return(emu, addr);
}
