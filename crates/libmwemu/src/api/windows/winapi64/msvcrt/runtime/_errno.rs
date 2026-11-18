use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _errno(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let errno_addr = match emu.maps.get_map_by_name("msvcrt_errno") {
        Some(m) => m.get_base(),
        None => {
            let addr = allocate_memory(emu, 8).unwrap_or(0);
            if addr != 0 {
                emu.maps.write_dword(addr, 0);
            }
            addr
        }
    };
    log_red!(emu, "msvcrt!_errno => 0x{:x}", errno_addr);
    emu.set_last_error(0);
    call.set_return(emu, errno_addr);
}
