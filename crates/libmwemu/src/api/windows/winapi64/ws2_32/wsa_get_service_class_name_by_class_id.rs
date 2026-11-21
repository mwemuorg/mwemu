use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAGetServiceClassNameByClassIdA(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdA");
}

fn wsa_get_service_class_name_by_class_id(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "ws2_32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let out_ptr = api_try!(emu, call.arg(emu, 2));
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(10109);
    call.set_return(emu, 0xFFFFFFFF);
}

pub fn WSAGetServiceClassNameByClassIdW(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdW");
}
