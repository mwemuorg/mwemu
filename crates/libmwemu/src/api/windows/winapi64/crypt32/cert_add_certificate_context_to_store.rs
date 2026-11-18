use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertAddCertificateContextToStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let store = api_try!(emu, call.arg(emu, 0));
    let cert_ctx = api_try!(emu, call.arg(emu, 1));
    let add_disp = api_try!(emu, call.arg(emu, 2));
    let out_ptr = api_try!(emu, call.arg(emu, 3));
    let hndl = helper::handler_create("cert_context");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, hndl);
    }
    log_red!(
        emu,
        "crypt32!CertAddCertificateContextToStore store:0x{:x} disp:{} => 0x{:x}",
        store,
        add_disp,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
