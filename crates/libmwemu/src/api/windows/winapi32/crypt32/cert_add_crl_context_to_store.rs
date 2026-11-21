use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertAddCRLContextToStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cert_store = api_try!(emu, call.arg(emu, 0));
    let p_crl_ctx = api_try!(emu, call.arg(emu, 1));
    let dw_add_disposition = api_try!(emu, call.arg(emu, 2));
    let pp_store_ctx = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "crypt32!CertAddCRLContextToStore 0x{:x}", h_cert_store);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
