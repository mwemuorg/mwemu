use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSASendMsg(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let msg_ptr = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    let bytes_sent_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSASendMsg sock:0x{:x} flags:0x{:x}",
        sock,
        flags
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        if bytes_sent_ptr != 0 {
            emu.maps.write_dword(bytes_sent_ptr, 0);
        }
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 6);
}
