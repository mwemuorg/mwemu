use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptMsgClose(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let msg = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!CryptMsgClose msg:0x{:x}", msg);
    if msg != 0 {
        helper::handler_close(msg);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
