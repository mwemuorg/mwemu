use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoDecrementMTAUsage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoDecrementMTAUsage arg1:0x{:x}", arg1);
    if helper::handler_exist(api_try!(emu, call.arg(emu, 0))) {
        helper::handler_close(api_try!(emu, call.arg(emu, 0)));
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
