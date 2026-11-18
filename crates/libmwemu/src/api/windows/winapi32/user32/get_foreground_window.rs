use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn GetForegroundWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!GetForegroundWindow");
    call.set_return(emu, 0x1337_0001);
}
