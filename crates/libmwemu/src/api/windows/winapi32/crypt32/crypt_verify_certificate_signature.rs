use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptVerifyCertificateSignature(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_crypt_prov = api_try!(emu, call.arg(emu, 0));
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 1));
    let pb_encoded = api_try!(emu, call.arg(emu, 2));
    let cb_encoded = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptVerifyCertificateSignature 0x{:x}",
        h_crypt_prov
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
