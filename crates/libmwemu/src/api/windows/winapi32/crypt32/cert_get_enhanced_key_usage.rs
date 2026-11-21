use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertGetEnhancedKeyUsage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_cert_ctx = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    let p_usage = api_try!(emu, call.arg(emu, 2));
    let pcb_usage = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "crypt32!CertGetEnhancedKeyUsage 0x{:x}", p_cert_ctx);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
