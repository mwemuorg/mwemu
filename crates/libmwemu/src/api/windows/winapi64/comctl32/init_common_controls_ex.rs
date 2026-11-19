use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn InitCommonControlsEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "comctl32!InitCommonControlsEx");
    // TODO: do something
    call.set_return(emu, 1); // TRUE
}
