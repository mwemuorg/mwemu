use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn sendto(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg_u32(emu, 0));
    let len = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(emu, "ws2_32!sendto sock: {} len: {}", sock, len);
    if !helper::socket_exist(sock as u64) {
        log::trace!("\tinvalid socket.");
    }
    call.set_return(emu, len as u64); // bytes sent
    call.cleanup_args(emu, 6);
}
