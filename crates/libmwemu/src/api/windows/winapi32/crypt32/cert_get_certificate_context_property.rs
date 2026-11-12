use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertGetCertificateContextProperty(emu: &mut emu::Emu) {
    let p_cert_ctx = WinApiAbi::arg(emu, 0);
    let dw_prop_id = WinApiAbi::arg(emu, 1);
    let pv_data = WinApiAbi::arg(emu, 2);
    let pcb_data = WinApiAbi::arg(emu, 3);
    log_red!(
        emu,
        "crypt32!CertGetCertificateContextProperty 0x{:x}",
        p_cert_ctx
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
