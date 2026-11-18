use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn SetCursor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!SetCursor");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}
