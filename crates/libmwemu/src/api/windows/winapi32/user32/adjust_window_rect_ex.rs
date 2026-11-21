use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn AdjustWindowRectEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!AdjustWindowRectEx");
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
