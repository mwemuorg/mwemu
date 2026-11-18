use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertGetCertificateChain(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _h_chain_engine = api_try!(emu, call.arg(emu, 0));
    let _p_cert_context = api_try!(emu, call.arg(emu, 1));
    let _p_time = api_try!(emu, call.arg(emu, 2));
    let _h_additional_store = api_try!(emu, call.arg(emu, 3));
    let _p_chain_para = api_try!(emu, call.arg(emu, 4));
    let _dw_flags = api_try!(emu, call.arg(emu, 5));
    let pp_chain_context = api_try!(emu, call.arg(emu, 6));

    let chain_h = helper::handler_create("cert_chain");
    log_red!(
        emu,
        "crypt32!CertGetCertificateChain => chain=0x{:x}",
        chain_h
    );
    if pp_chain_context != 0 {
        emu.maps.write_qword(pp_chain_context, chain_h);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
