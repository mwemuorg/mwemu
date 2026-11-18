use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FileTimeToSystemTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file_time = api_try!(emu, call.arg_u32(emu, 0));
    let sys_time_ptr = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!FileTimeToSystemTime");

    call.cleanup_args(emu, 2);

    call.set_return(emu, 1);
}
