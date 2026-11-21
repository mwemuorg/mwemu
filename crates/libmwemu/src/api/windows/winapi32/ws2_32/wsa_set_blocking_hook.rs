use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSASetBlockingHook(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hook_fn = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WSASetBlockingHook hook:0x{:x}", hook_fn);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}
