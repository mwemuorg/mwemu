use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn __setlc_active(emu: &mut emu::Emu, addr: u32, api: &str) {
    let call = WinApi(Cdecl);
    let addr = allocate_memory(emu, 4).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_dword(addr, 0);
    }
    log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
    call.set_return(emu, addr);
}
