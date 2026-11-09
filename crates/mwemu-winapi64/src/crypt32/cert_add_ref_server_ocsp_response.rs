use crate::emu;
use crate::winapi::helper;

pub fn CertAddRefServerOcspResponse(emu: &mut emu::Emu) {
    let cert_ctx = emu.regs().rcx;
    log_red!(
        emu,
        "crypt32!CertAddRefServerOcspResponse ctx:0x{:x}",
        cert_ctx
    );
    let handle = helper::handler_create("CertAddRefServerOcspResponse");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
