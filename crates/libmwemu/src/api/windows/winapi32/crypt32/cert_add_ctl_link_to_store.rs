use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertAddCTLLinkToStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cert_ctx = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!CertAddCTLLinkToStore ctx:0x{:x}", cert_ctx);
    let handle = helper::handler_create("CertAddCTLLinkToStore");
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
