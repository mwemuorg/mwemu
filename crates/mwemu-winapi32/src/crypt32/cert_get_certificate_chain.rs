use crate::emu;
use crate::winapi::helper;

pub fn CertGetCertificateChain(emu: &mut emu::Emu) {
    let engine = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let time = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let add_store = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let chain_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    let chain_ctx_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
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
    emu.regs_mut().rax = 1;
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
}
