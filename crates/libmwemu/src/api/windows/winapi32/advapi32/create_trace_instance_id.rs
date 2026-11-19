use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateTraceInstanceId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    let hndl = helper::handler_create("CreateTraceInstanceId");
    log_red!(emu, "advapi32!CreateTraceInstanceId => 0x{:x}", hndl);
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 2);
}
