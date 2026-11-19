use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertGetCertificateContextProperty(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_cert_ctx = api_try!(emu, call.arg(emu, 0));
    let dw_prop_id = api_try!(emu, call.arg(emu, 1));
    let pv_data = api_try!(emu, call.arg(emu, 2));
    let pcb_data = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CertGetCertificateContextProperty 0x{:x}",
        p_cert_ctx
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
