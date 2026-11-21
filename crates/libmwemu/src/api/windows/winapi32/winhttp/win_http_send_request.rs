use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn WinHttpSendRequest(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let request = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "winhttp!WinHttpSendRequest request: 0x{:x}", request);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 7);
}
