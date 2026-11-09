use crate::emu;
use crate::winapi::helper;

pub fn CertCreateCTLEntryFromCertificateContextProperties(emu: &mut emu::Emu) {
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CertCreateCTLEntryFromCertificateContextProperties"
    );
    emu.regs_mut().rax =
        helper::handler_create("CertCreateCTLEntryFromCertificateContextProperties");
}
