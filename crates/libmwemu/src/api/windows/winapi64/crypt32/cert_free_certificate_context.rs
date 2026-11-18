use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertFreeCertificateContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cert_ctx = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext ctx: 0x{:x}",
        cert_ctx
    );

    call.set_return(emu, 1);
}
