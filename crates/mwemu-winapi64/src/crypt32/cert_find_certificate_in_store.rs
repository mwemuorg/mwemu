use crate::emu;

pub fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let h_store = emu.regs().rcx;
    let encoding_type = emu.regs().rdx;
    let find_flags = emu.regs().r8;
    let find_type = emu.regs().r9;
    let find_para = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let prev_cert = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    emu.regs_mut().rax = 0;
}
