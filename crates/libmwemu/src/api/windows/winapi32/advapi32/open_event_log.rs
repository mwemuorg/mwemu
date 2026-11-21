use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenEventLogA(emu: &mut emu::Emu) {
    open_event_log(emu, "OpenEventLogA");
}

fn open_event_log(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "advapi32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    call.cleanup_args(emu, 2);
    let hndl = helper::handler_create(&format!("advapi32://{api}"));
    call.set_return(emu, hndl);
}

pub fn OpenEventLogW(emu: &mut emu::Emu) {
    open_event_log(emu, "OpenEventLogW");
}
