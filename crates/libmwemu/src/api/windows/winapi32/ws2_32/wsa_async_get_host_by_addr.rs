use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAAsyncGetHostByAddr(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let msg = api_try!(emu, call.arg_u32(emu, 1));
    let addr_ptr = api_try!(emu, call.arg(emu, 2));
    let addr_len = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSAAsyncGetHostByAddr hwnd:0x{:x} msg:{}",
        hwnd,
        msg
    );
    let handle = helper::handler_create("wsa_async_task");
    call.set_return(emu, handle);
    call.cleanup_args(emu, 7);
}
