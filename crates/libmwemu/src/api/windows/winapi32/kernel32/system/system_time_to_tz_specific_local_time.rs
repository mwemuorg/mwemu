use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let tz_ptr = api_try!(emu, call.arg_u32(emu, 0));
    let ut_ptr = api_try!(emu, call.arg_u32(emu, 1));
    let lt_ptr = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!SystemTimeToTzSpecificLocalTime");

    call.cleanup_args(emu, 3);

    call.set_return(emu, 1);
}
