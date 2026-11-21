use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertGetIssuerCertificateFromStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cert_store = api_try!(emu, call.arg(emu, 0));
    let p_subject_ctx = api_try!(emu, call.arg(emu, 1));
    let p_prev_issuer_ctx = api_try!(emu, call.arg(emu, 2));
    let pdw_flags = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CertGetIssuerCertificateFromStore 0x{:x}",
        h_cert_store
    );
    let hndl = helper::handler_create("CertGetIssuerCertificateFromStore");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 4);
}
