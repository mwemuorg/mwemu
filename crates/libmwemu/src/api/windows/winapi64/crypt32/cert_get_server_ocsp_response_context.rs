use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertGetServerOcspResponseContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let server_ocsp_status = api_try!(emu, call.arg(emu, 0));
    let _flags = api_try!(emu, call.arg(emu, 1));
    let _reserved = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CertGetServerOcspResponseContext status:0x{:x}",
        server_ocsp_status
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("ocsp_response_ctx");
    call.set_return(emu, handle);
}
