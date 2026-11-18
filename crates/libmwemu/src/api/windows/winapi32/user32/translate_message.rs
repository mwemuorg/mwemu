use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn TranslateMessage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!TranslateMessage");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}
