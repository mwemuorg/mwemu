use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoSetMessageDispatcher(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dispatcher = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "ole32!CoSetMessageDispatcher dispatcher:0x{:x}",
        dispatcher
    );
    if dispatcher != 0 {
        let _ = helper::handler_create("CoMessageDispatcher");
    }
    emu.set_last_error(0);
}
