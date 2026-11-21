use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertFreeCertificateChain(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let chain_ctx = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(chain_ctx);
    log_red!(emu, "crypt32!CertFreeCertificateChain 0x{:x}", chain_ctx);
    call.cleanup_args(emu, 1);
}
