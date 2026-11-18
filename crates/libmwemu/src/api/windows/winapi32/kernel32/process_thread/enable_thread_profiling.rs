use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn EnableThreadProfiling(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _ThreadHandle = api_try!(emu, call.arg_u32(emu, 0));
    let _Flags = api_try!(emu, call.arg_u32(emu, 1));
    let _HardwareCounters = api_try!(emu, call.arg_u32(emu, 2));
    let _PerformanceDataHandle = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "kernel32!EnableThreadProfiling");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}
