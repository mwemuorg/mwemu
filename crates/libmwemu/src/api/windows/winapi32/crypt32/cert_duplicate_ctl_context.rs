use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertDuplicateCTLContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    call.cleanup_args(emu, 1);
    log_red!(emu, "crypt32!CertDuplicateCTLContext arg1:0x{:x}", arg1);
    call.set_return(emu, helper::handler_create("CertDuplicateCTLContext"));
}
