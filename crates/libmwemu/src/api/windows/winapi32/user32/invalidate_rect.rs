use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn InvalidateRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!InvalidateRect");
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
