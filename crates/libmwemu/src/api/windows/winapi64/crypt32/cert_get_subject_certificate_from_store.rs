use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertGetSubjectCertificateFromStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let store = api_try!(emu, call.arg(emu, 0));
    let cert_encoding = api_try!(emu, call.arg_u32(emu, 1));
    let cert_id = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CertGetSubjectCertificateFromStore store:0x{:x} enc:0x{:x}",
        store,
        cert_encoding
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("cert_subject_ctx");
    call.set_return(emu, handle);
}
