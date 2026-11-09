use crate::emu;
use crate::winapi::helper;

pub fn CertAddCertificateContextToStore(emu: &mut emu::Emu) {
    let store = emu.regs().rcx;
    let cert_ctx = emu.regs().rdx;
    let add_disp = emu.regs().r8;
    let out_ptr = emu.regs().r9;
    let hndl = helper::handler_create("cert_context");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, hndl);
    }
    log_red!(
        emu,
        "crypt32!CertAddCertificateContextToStore store:0x{:x} disp:{} => 0x{:x}",
        store,
        add_disp,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
