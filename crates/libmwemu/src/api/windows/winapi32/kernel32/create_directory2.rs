use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateDirectory2A(emu: &mut emu::Emu) {
    create_directory2(emu, "CreateDirectory2A");
}

fn create_directory2(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn CreateDirectory2W(emu: &mut emu::Emu) {
    create_directory2(emu, "CreateDirectory2W");
}
