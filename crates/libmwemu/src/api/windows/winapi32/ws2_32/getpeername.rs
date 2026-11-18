use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn getpeername(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ws2_32!getpeername sock: {}", sock);
    call.set_return(emu, 0xffff_ffff_u64); // SOCKET_ERROR
    call.cleanup_args(emu, 3);
}
