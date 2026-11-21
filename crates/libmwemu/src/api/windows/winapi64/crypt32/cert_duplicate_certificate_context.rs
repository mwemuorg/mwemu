use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertDuplicateCertificateContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let context = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "crypt32!CertDuplicateCertificateContext ctx:0x{:x}",
        context
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("certduplicatecertificatecontext");
    call.set_return(emu, handle);
}
