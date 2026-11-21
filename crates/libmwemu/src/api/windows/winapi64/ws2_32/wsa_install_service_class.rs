use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAInstallServiceClassA(emu: &mut emu::Emu) {
    wsa_install_service_class(emu, "WSAInstallServiceClassA");
}

fn wsa_install_service_class(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let class_info_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!{api} ptr:0x{:x}", class_info_ptr);
    let handle = helper::handler_create(api);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

pub fn WSAInstallServiceClassW(emu: &mut emu::Emu) {
    wsa_install_service_class(emu, "WSAInstallServiceClassW");
}
