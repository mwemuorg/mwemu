use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OleIsCurrentClipboard(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pdata_obj = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "ole32!OleIsCurrentClipboard pDataObj:0x{:x}",
        pdata_obj
    );
    let result = if helper::handler_exist(pdata_obj) {
        0u64
    } else {
        1u64
    };
    emu.set_last_error(0);
    call.set_return(emu, result);
}
