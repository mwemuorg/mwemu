use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn BeginPaint(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!BeginPaint");
    call.set_return(emu, 0x1337_0002);
    call.cleanup_args(emu, 2);
}
