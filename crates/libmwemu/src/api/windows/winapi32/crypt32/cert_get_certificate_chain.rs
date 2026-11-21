use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertGetCertificateChain(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let engine = api_try!(emu, call.arg(emu, 0));
    let cert_ctx = api_try!(emu, call.arg(emu, 1));
    let time = api_try!(emu, call.arg(emu, 2));
    let add_store = api_try!(emu, call.arg(emu, 3));
    let chain_para = api_try!(emu, call.arg(emu, 4));
    let flags = api_try!(emu, call.arg(emu, 5));
    let _reserved = api_try!(emu, call.arg(emu, 6));
    let chain_ctx_ptr = api_try!(emu, call.arg(emu, 7));
    let hndl = helper::handler_create("cert_chain");
    if chain_ctx_ptr != 0 {
        emu.maps.write_dword(chain_ctx_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "crypt32!CertGetCertificateChain cert:0x{:x} => 0x{:x}",
        cert_ctx,
        hndl
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}
