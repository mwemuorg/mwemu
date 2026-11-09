use crate::emu;
use crate::winapi::helper;

pub fn CertAddEncodedCertificateToStore(emu: &mut emu::Emu) {
    let store = emu.regs().rcx;
    let encoding = emu.regs().rdx;
    let cert_data = emu.regs().r8;
    let cert_len = emu.regs().r9;
    let add_disp = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let hndl = helper::handler_create("cert_encoded");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, hndl);
    }
    log_red!(
        emu,
        "crypt32!CertAddEncodedCertificateToStore store:0x{:x} len:{}",
        store,
        cert_len
    );
    emu.regs_mut().rax = 1;
}
