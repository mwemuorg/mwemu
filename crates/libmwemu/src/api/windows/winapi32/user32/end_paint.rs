use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn EndPaint(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!EndPaint");
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
