use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn WSASetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ws2_32!WSASetLastError err: {}", err);
    emu.set_last_error(err as u64);
    call.cleanup_args(emu, 1);
}
