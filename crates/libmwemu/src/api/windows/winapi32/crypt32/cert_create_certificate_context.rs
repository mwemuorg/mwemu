use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertCreateCertificateContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 0));
    let pb_cert_encoded = api_try!(emu, call.arg(emu, 1));
    let cb_cert_encoded = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CertCreateCertificateContext dw_cert_encoding:0x{:x} pb_cert_encoded:0x{:x} cb_cert_encoded:0x{:x}",
        dw_cert_encoding,
        pb_cert_encoded,
        cb_cert_encoded
    );
    let hndl = helper::handler_create("CertCreateCertificateContext");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 3);
}
