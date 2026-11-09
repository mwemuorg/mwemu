use crate::emu;
use crate::winapi::helper;

pub fn CertGetSubjectCertificateFromStore(emu: &mut emu::Emu) {
    let store = emu.regs().rcx;
    let cert_encoding = emu.regs().rdx as u32;
    let cert_id = emu.regs().r8;
    log_red!(
        emu,
        "crypt32!CertGetSubjectCertificateFromStore store:0x{:x} enc:0x{:x}",
        store,
        cert_encoding
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("cert_subject_ctx");
    emu.regs_mut().rax = handle;
}
