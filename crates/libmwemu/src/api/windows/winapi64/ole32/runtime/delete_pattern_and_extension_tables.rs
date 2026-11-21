use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DeletePatternAndExtensionTables(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(handle);
    log_red!(
        emu,
        "ole32!DeletePatternAndExtensionTables handle:0x{:x}",
        handle
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
