use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileNameTransactedW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 5);
    log_red!(emu, "kernel32!FindFirstFileNameTransactedW");
    call.set_return(emu, helper::handler_create("FindFirstFileNameTransactedW"));
}
