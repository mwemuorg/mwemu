use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn PeekMessageA(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageA");
}

fn peek_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!{api}");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 5);
}

pub(super) fn PeekMessageW(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageW");
}
