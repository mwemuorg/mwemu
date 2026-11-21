use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertGetIssuerCertificateFromStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let store = api_try!(emu, call.arg(emu, 0));
    let subject_ctx = api_try!(emu, call.arg(emu, 1));
    let prev_issuer = api_try!(emu, call.arg(emu, 2));
    let flags_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CertGetIssuerCertificateFromStore store:0x{:x} subj:0x{:x}",
        store,
        subject_ctx
    );
    emu.set_last_error(0);
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    let handle = helper::handler_create("cert_issuer_ctx");
    call.set_return(emu, handle);
}
