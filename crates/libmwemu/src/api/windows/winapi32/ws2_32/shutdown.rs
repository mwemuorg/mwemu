use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn shutdown(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let how = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "ws2_32!shutdown sock:{} how:{}", sock, how);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xffff_ffff_u64);
    } else {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 2);
}
