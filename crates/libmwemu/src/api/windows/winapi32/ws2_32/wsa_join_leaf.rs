use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAJoinLeaf(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let namelen = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(emu, "ws2_32!WSAJoinLeaf sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(0);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        call.set_return(emu, sock);
    }
    call.cleanup_args(emu, 8);
}
