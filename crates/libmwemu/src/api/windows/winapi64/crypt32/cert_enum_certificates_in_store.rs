use crate::emu;

pub fn CertEnumCertificatesInStore(emu: &mut emu::Emu) {
    let store = emu.regs().rcx;
    let prev_ctx = emu.regs().rdx;
    log_red!(
        emu,
        "crypt32!CertEnumCertificatesInStore store:0x{:x} prev:0x{:x}",
        store,
        prev_ctx
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
