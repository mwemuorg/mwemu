use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertFreeCertificateChain(emu: &mut emu::Emu) {
    let chain_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(chain_ctx);
    log_red!(emu, "crypt32!CertFreeCertificateChain 0x{:x}", chain_ctx);
    stdcall_ret(emu, 1);
}
