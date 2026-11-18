use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAHtonl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let hostlong = api_try!(emu, call.arg_u32(emu, 1));
    let result_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!WSAHtonl sock:0x{:x} val:{}", sock, hostlong);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        if result_ptr != 0 {
            emu.maps.write_dword(result_ptr, hostlong.to_be());
        }
        call.set_return(emu, 0);
    }
}
