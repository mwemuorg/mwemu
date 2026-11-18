use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptVerifyCertificateSignatureEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_crypt_prov = api_try!(emu, call.arg(emu, 0));
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 1));
    let dw_subject_type = api_try!(emu, call.arg(emu, 2));
    let pv_subject = api_try!(emu, call.arg(emu, 3));
    let dw_issuer_type = api_try!(emu, call.arg(emu, 4));
    let pv_issuer = api_try!(emu, call.arg(emu, 5));
    let dw_flags = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "crypt32!CryptVerifyCertificateSignatureEx 0x{:x}",
        h_crypt_prov
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}
