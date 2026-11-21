use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn DispatchMessageA(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageA");
}

fn dispatch_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!{api}");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub(super) fn DispatchMessageW(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageW");
}
