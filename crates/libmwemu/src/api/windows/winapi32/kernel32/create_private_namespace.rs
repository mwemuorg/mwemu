use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreatePrivateNamespaceA(emu: &mut emu::Emu) {
    create_private_namespace(emu, "CreatePrivateNamespaceA");
}

fn create_private_namespace(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 3);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn CreatePrivateNamespaceW(emu: &mut emu::Emu) {
    create_private_namespace(emu, "CreatePrivateNamespaceW");
}
