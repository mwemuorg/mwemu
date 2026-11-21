use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSARecvDisconnect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let data_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ws2_32!WSARecvDisconnect sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 2);
}
