use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertGetServerOcspResponseContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    call.cleanup_args(emu, 3);
    log_red!(
        emu,
        "crypt32!CertGetServerOcspResponseContext arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    call.set_return(
        emu,
        helper::handler_create("CertGetServerOcspResponseContext"),
    );
}
