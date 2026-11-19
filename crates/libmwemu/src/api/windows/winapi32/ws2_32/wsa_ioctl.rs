use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAIoctl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let ioctl_code = api_try!(emu, call.arg_u32(emu, 1));
    let in_buf_ptr = api_try!(emu, call.arg(emu, 2));
    let in_buf_len = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSAIoctl sock:0x{:x} code:0x{:x}",
        sock,
        ioctl_code
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 9);
}
