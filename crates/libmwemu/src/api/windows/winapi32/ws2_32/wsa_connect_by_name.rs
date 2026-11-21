use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAConnectByNameA(emu: &mut emu::Emu) {
    wsa_connect_by_name(emu, "WSAConnectByNameA", false);
}

fn wsa_connect_by_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let node_ptr = api_try!(emu, call.arg(emu, 1));
    let service_ptr = api_try!(emu, call.arg(emu, 2));
    let node = if node_ptr != 0 {
        strings::read_text(emu, node_ptr, wide)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        strings::read_text(emu, service_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!{api} sock:0x{:x} node:'{}' service:'{}'",
        sock,
        node,
        service
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0);
    } else {
        call.set_return(emu, 1);
    }
    call.cleanup_args(emu, 9);
}

pub fn WSAConnectByNameW(emu: &mut emu::Emu) {
    wsa_connect_by_name(emu, "WSAConnectByNameW", true);
}
