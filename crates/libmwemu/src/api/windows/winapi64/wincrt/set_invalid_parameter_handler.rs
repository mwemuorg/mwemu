use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn set_invalid_parameter_handler(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    log_red!(emu, "wincrt!_set_invalid_parameter_handler");
    call.set_return(emu, 0);
}
