use crate::emu;

pub fn CertFreeCertificateContext(emu: &mut emu::Emu) {
    let cert_ctx = emu.regs().rcx;

    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext ctx: 0x{:x}",
        cert_ctx
    );

    emu.regs_mut().rax = 1;
}
