use crate::emu;
use crate::winapi::helper;

pub fn CertFreeCertificateChain(emu: &mut emu::Emu) {
    let chain_ctx = emu.regs().rcx;
    helper::handler_close(chain_ctx);
    log_red!(emu, "crypt32!CertFreeCertificateChain 0x{:x}", chain_ctx);
}
