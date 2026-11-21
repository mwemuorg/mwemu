use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSADuplicateSocketA(emu: &mut emu::Emu) {
    wsa_duplicate_socket(emu, "WSADuplicateSocketA");
}

fn wsa_duplicate_socket(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let process_id = api_try!(emu, call.arg_u32(emu, 1));
    let proto_info_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!{api} sock:0x{:x} pid:{}", sock, process_id);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 3);
}

pub fn WSADuplicateSocketW(emu: &mut emu::Emu) {
    wsa_duplicate_socket(emu, "WSADuplicateSocketW");
}
