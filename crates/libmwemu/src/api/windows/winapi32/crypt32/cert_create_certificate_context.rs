use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertCreateCertificateContext(emu: &mut emu::Emu) {
    let dw_cert_encoding = WinApiAbi::arg(emu, 0);
    let pb_cert_encoded = WinApiAbi::arg(emu, 1);
    let cb_cert_encoded = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CertCreateCertificateContext dw_cert_encoding:0x{:x} pb_cert_encoded:0x{:x} cb_cert_encoded:0x{:x}",
        dw_cert_encoding,
        pb_cert_encoded,
        cb_cert_encoded
    );
    let hndl = helper::handler_create("CertCreateCertificateContext");
    emu.regs_mut().rax = hndl;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
