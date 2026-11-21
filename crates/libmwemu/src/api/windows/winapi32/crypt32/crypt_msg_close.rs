use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptMsgClose(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let msg = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(msg);
    log_red!(emu, "crypt32!CryptMsgClose 0x{:x}", msg);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
