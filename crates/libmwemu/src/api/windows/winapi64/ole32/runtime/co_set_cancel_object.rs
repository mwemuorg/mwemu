use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoSetCancelObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let punk = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoSetCancelObject punk:0x{:x}", punk);
    if punk != 0 {
        let _ = helper::handler_create("CoCancelObject");
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
