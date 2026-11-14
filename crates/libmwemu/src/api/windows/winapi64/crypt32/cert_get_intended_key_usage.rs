use crate::emu;

pub fn CertGetIntendedKeyUsage(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CertGetIntendedKeyUsage arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    if arg4 != 0 {
        emu.maps.write_dword(arg4, 0);
    }
    emu.regs_mut().rax = 1;
}
