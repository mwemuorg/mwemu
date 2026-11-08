use crate::emu;
use crate::winapi::helper;

pub fn CertGetIssuerCertificateFromStore(emu: &mut emu::Emu) {
    let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_subject_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_prev_issuer_ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pdw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CertGetIssuerCertificateFromStore 0x{:x}",
        h_cert_store
    );
    let hndl = helper::handler_create("CertGetIssuerCertificateFromStore");
    emu.regs_mut().rax = hndl;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
