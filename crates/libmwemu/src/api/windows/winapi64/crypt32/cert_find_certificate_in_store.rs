use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_store = api_try!(emu, call.arg(emu, 0));
    let encoding_type = api_try!(emu, call.arg(emu, 1));
    let find_flags = api_try!(emu, call.arg(emu, 2));
    let find_type = api_try!(emu, call.arg(emu, 3));
    let find_para = api_try!(emu, call.arg(emu, 4));
    let prev_cert = api_try!(emu, call.arg(emu, 5));

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    call.set_return(emu, 0);
}
