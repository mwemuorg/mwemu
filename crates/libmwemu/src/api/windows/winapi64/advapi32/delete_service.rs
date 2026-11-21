use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DeleteService(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hsvc = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(hsvc);
    log_red!(emu, "advapi32!DeleteService 0x{:x}", hsvc);
    call.set_return(emu, 1);
}
