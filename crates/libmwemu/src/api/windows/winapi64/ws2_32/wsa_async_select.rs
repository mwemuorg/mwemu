use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAAsyncSelect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let hwnd = api_try!(emu, call.arg(emu, 1));
    let msg = api_try!(emu, call.arg(emu, 2));
    let events = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSAAsyncSelect sock:0x{:x} hwnd:0x{:x} msg:{} events:0x{:x}",
        sock,
        hwnd,
        msg,
        events
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        call.set_return(emu, 0);
    }
}
