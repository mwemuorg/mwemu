use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAGetServiceClassInfoA(emu: &mut emu::Emu) {
    wsa_get_service_class_info(emu, "WSAGetServiceClassInfoA");
}

fn wsa_get_service_class_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let provider_id = api_try!(emu, call.arg(emu, 0));
    let class_id = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10109);
    call.set_return(emu, 0xFFFFFFFF);
}

pub fn WSAGetServiceClassInfoW(emu: &mut emu::Emu) {
    wsa_get_service_class_info(emu, "WSAGetServiceClassInfoW");
}
