use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn GetKeyboardLayout(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!GetKeyboardLayout");
    call.set_return(emu, 0x0409_0409);
    call.cleanup_args(emu, 1);
}
