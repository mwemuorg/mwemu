use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAPoll(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let fds_ptr = api_try!(emu, call.arg(emu, 0));
    let nfds = api_try!(emu, call.arg_u32(emu, 1));
    let timeout = api_try!(emu, call.arg_i32(emu, 2));
    log_red!(emu, "ws2_32!WSAPoll nfds:{} timeout:{}", nfds, timeout);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}
