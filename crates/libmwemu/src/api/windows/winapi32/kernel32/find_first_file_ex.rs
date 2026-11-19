use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileExA(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExA");
}

fn find_first_file_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 6);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn FindFirstFileExW(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExW");
}
