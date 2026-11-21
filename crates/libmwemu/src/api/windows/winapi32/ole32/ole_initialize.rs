use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::errors::*;

pub(super) fn OleInitialize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "ole32!OleInitialize");
    call.set_return(emu, S_OK);
    call.cleanup_args(emu, 1);
}
