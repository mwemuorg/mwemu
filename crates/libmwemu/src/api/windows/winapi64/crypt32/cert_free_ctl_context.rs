use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertFreeCTLContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let context = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!CertFreeCTLContext ctx:0x{:x}", context);
    if context != 0 {
        helper::handler_close(context);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
