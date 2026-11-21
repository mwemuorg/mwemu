use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WinHttpSendRequest(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_request = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "winhttp!WinHttpSendRequest request: 0x{:x}", h_request);

    if !helper::handler_exist(h_request) {
        log::trace!("\tinvalid request handle");
    }

    call.set_return(emu, 1);
}
