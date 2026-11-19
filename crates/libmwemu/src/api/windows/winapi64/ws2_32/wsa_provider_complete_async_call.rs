use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAProviderCompleteAsyncCall(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let id = api_try!(emu, call.arg(emu, 0));
    let result = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "ws2_32!WSAProviderCompleteAsyncCall id:0x{:x} result:{}",
        id,
        result
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
