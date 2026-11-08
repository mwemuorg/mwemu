use crate::emu;
use crate::winapi::helper;

pub fn CertGetServerOcspResponseContext(emu: &mut emu::Emu) {
    let server_ocsp_status = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    log_red!(
        emu,
        "crypt32!CertGetServerOcspResponseContext status:0x{:x}",
        server_ocsp_status
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("ocsp_response_ctx");
    emu.regs_mut().rax = handle;
}
