use crate::emu;

pub fn CertSelectCertificateChains(emu: &mut emu::Emu) {
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CertSelectCertificateChains ctx:0x{:x}",
        cert_ctx
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
