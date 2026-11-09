use crate::emu;
use crate::winapi::helper;

pub fn CertGetIssuerCertificateFromStore(emu: &mut emu::Emu) {
    let store = emu.regs().rcx;
    let subject_ctx = emu.regs().rdx;
    let prev_issuer = emu.regs().r8;
    let flags_ptr = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CertGetIssuerCertificateFromStore store:0x{:x} subj:0x{:x}",
        store,
        subject_ctx
    );
    emu.set_last_error(0);
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    let handle = helper::handler_create("cert_issuer_ctx");
    emu.regs_mut().rax = handle;
}
