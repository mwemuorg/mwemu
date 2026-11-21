use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DeregisterEventSource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(hndl) {
        helper::handler_close(hndl);
    }
    log_red!(emu, "advapi32!DeregisterEventSource 0x{:x}", hndl);
    call.set_return(emu, 1);
}
