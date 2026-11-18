use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAGetServiceClassNameByClassIdA(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdA");
}

fn wsa_get_service_class_name_by_class_id(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let class_id_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10109);
    call.set_return(emu, 0xFFFFFFFF);
    call.cleanup_args(emu, 3);
}

pub fn WSAGetServiceClassNameByClassIdW(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdW");
}
