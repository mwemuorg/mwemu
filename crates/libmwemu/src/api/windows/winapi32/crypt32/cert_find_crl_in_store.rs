use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertFindCRLInStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cert_store = api_try!(emu, call.arg(emu, 0));
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 1));
    let dw_find_flags = api_try!(emu, call.arg(emu, 2));
    let dw_find_type = api_try!(emu, call.arg(emu, 3));
    let pv_find_para = api_try!(emu, call.arg(emu, 4));
    let p_prev_crl_ctx = api_try!(emu, call.arg(emu, 5));
    log_red!(emu, "crypt32!CertFindCRLInStore 0x{:x}", h_cert_store);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
