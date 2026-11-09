use crate::emu;
use crate::winapi::helper;

pub fn CertCompareCertificate(emu: &mut emu::Emu) {
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_cert_id1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_cert_id2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CertCompareCertificate dw_cert_encoding:0x{:x} p_cert_id1:0x{:x} p_cert_id2:0x{:x}",
        dw_cert_encoding,
        p_cert_id1,
        p_cert_id2
    );
    emu.set_last_error(0);
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
