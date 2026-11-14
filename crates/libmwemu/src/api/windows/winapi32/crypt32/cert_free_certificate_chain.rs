use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertFreeCertificateChain(emu: &mut emu::Emu) {
    let chain_ctx = WinApiAbi::arg(emu, 0);
    helper::handler_close(chain_ctx);
    log_red!(emu, "crypt32!CertFreeCertificateChain 0x{:x}", chain_ctx);
    stdcall_ret(emu, 1);
}
