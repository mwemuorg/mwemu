use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn _get_initial_narrow_environment(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let env_addr = allocate_memory(emu, 8).unwrap_or(0);
    if env_addr != 0 {
        emu.maps.write_dword(env_addr, 0); // NULL-terminated empty environment
    }
    log_red!(
        emu,
        "msvcrt!_get_initial_narrow_environment => 0x{:x}",
        env_addr
    );
    call.set_return(emu, env_addr);
}
