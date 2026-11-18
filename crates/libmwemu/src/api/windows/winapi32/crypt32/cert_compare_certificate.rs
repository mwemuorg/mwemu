use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertCompareCertificate(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 0));
    let p_cert_id1 = api_try!(emu, call.arg(emu, 1));
    let p_cert_id2 = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CertCompareCertificate dw_cert_encoding:0x{:x} p_cert_id1:0x{:x} p_cert_id2:0x{:x}",
        dw_cert_encoding,
        p_cert_id1,
        p_cert_id2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
