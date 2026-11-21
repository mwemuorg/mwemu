use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoReleaseMarshalData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoReleaseMarshalData arg1:0x{:x}", arg1);
    if arg1 != 0 {
        helper::handler_close(arg1);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
