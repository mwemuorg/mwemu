use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateFileTransactedA(emu: &mut emu::Emu) {
    create_file_transacted(emu, "CreateFileTransactedA");
}

fn create_file_transacted(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 10);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn CreateFileTransactedW(emu: &mut emu::Emu) {
    create_file_transacted(emu, "CreateFileTransactedW");
}
