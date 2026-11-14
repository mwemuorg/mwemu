use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertCompareCertificateName(emu: &mut emu::Emu) {
    let dw_cert_encoding = WinApiAbi::arg(emu, 0);
    let p_name1 = WinApiAbi::arg(emu, 1);
    let p_name2 = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CertCompareCertificateName dw_cert_encoding:0x{:x} p_name1:0x{:x} p_name2:0x{:x}",
        dw_cert_encoding,
        p_name1,
        p_name2
    );
    emu.set_last_error(0);
    let h = WinApiAbi::arg(emu, 0);
    emu.regs_mut().rax = 1;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
