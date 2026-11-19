use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ioctlsocket(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let cmd = api_try!(emu, call.arg_u32(emu, 1));
    let argp_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!ioctlsocket sock:0x{:x} cmd:0x{:x}", sock, cmd);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        if argp_ptr != 0 && cmd == 0x4004667f {
            emu.maps.write_dword(argp_ptr, 0);
        }
        call.set_return(emu, 0);
    }
}
