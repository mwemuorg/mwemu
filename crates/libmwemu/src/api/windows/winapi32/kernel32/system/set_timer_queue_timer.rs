use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetTimerQueueTimer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _TimerQueue = api_try!(emu, call.arg_u32(emu, 0));
    let _Callback = api_try!(emu, call.arg_u32(emu, 1));
    let _Parameter = api_try!(emu, call.arg_u32(emu, 2));
    let _DueTime = api_try!(emu, call.arg_u32(emu, 3));
    let _Period = api_try!(emu, call.arg_u32(emu, 4));
    let _PreferIo = api_try!(emu, call.arg_u32(emu, 5));

    log_red!(emu, "kernel32!SetTimerQueueTimer");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}
