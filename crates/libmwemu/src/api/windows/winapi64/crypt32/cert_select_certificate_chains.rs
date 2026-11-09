use crate::emu;

pub fn CertSelectCertificateChains(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    log_red!(
        emu,
        "crypt32!CertSelectCertificateChains 0x{:x} 0x{:x}",
        arg1,
        arg2
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
