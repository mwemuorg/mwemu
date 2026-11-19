use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAWaitForMultipleEvents(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let count = api_try!(emu, call.arg_u32(emu, 0));
    let events_ptr = api_try!(emu, call.arg(emu, 1));
    let wait_all = api_try!(emu, call.arg_u32(emu, 2));
    let timeout = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSAWaitForMultipleEvents count:{} wait_all:{} timeout:{}",
        count,
        wait_all,
        timeout
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 5);
}
