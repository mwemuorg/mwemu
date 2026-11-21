use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileTransactedA(emu: &mut emu::Emu) {
    find_first_file_transacted(emu, "FindFirstFileTransactedA");
}

fn find_first_file_transacted(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 7);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn FindFirstFileTransactedW(emu: &mut emu::Emu) {
    find_first_file_transacted(emu, "FindFirstFileTransactedW");
}
