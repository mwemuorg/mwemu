use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _get_initial_narrow_environment(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let env_addr = allocate_memory(emu, 16).unwrap_or(0);
    if env_addr != 0 {
        emu.maps.write_qword(env_addr, 0);
    }
    log_red!(
        emu,
        "msvcrt!_get_initial_narrow_environment => 0x{:x}",
        env_addr
    );
    emu.set_last_error(0);
    call.set_return(emu, env_addr);
}
