use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSACancelAsyncRequest(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let async_handle = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "ws2_32!WSACancelAsyncRequest handle:0x{:x}",
        async_handle
    );
    if helper::handler_exist(async_handle) {
        helper::handler_close(async_handle);
        call.set_return(emu, 0);
    } else {
        emu.set_last_error(10044);
        call.set_return(emu, 0xFFFFFFFF);
    }
    call.cleanup_args(emu, 1);
}
