use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn _initialize_narrow_environment(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    log_red!(emu, "wincrt!_initialize_narrow_environment");
    call.set_return(emu, 0);
}
