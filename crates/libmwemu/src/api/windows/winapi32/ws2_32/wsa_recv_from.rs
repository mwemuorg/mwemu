use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

pub fn WSARecvFrom(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let buffers_ptr = api_try!(emu, call.arg(emu, 1));
    let buffer_count = api_try!(emu, call.arg_u32(emu, 2));
    let bytes_recvd_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSARecvFrom sock:0x{:x} buffers:{}",
        sock,
        buffer_count
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(WSAENOTSOCK);
        call.set_return(emu, SOCKET_ERROR);
    } else {
        if bytes_recvd_ptr != 0 {
            emu.maps.write_dword(bytes_recvd_ptr, 0);
        }
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 9);
}
