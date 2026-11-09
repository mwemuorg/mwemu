use crate::emu;
use crate::winapi::helper;

pub fn CertCreateCertificateContext(emu: &mut emu::Emu) {
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pb_cert_encoded = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cb_cert_encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
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
