use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertGetIssuerCertificateFromStore(emu: &mut emu::Emu) {
    let h_cert_store = WinApiAbi::arg(emu, 0);
    let p_subject_ctx = WinApiAbi::arg(emu, 1);
    let p_prev_issuer_ctx = WinApiAbi::arg(emu, 2);
    let pdw_flags = WinApiAbi::arg(emu, 3);
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
