use super::allocate_memory;
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn _get_initial_narrow_environment(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let env_ptr = match allocate_memory(emu, 8) {
        Some(a) => a,
        None => {
            call.set_return(emu, 0);
            return;
        }
    };
    emu.maps.write_qword(env_ptr, 0);
    log_red!(
        emu,
        "wincrt!_get_initial_narrow_environment => 0x{:x}",
        env_ptr
    );
    call.set_return(emu, env_ptr);
}
