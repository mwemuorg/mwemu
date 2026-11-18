use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertAddEncodedCertificateToStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let store = api_try!(emu, call.arg(emu, 0));
    let encoding = api_try!(emu, call.arg(emu, 1));
    let cert_data = api_try!(emu, call.arg(emu, 2));
    let cert_len = api_try!(emu, call.arg(emu, 3));
    let add_disp = api_try!(emu, call.arg(emu, 4));
    let out_ptr = api_try!(emu, call.arg(emu, 5));
    let hndl = helper::handler_create("cert_encoded");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, hndl);
    }
    log_red!(
        emu,
        "crypt32!CertAddEncodedCertificateToStore store:0x{:x} len:{}",
        store,
        cert_len
    );
    call.set_return(emu, 1);
}
