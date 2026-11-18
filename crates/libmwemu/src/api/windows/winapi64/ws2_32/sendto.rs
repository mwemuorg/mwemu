use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn sendto(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let _buf = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let _flags = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(emu, "ws2_32!sendto sock:0x{:x} len:{}", sock, len);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFFFFFFFFFF);
    } else {
        call.set_return(emu, len);
    }
}
