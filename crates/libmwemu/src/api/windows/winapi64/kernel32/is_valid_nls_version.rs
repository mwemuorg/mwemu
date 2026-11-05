use crate::emu;

pub fn IsValidNLSVersion(emu: &mut emu::Emu) {
    let func = emu.regs().rcx;
    let lp_locale_name = emu.regs().rdx;
    let lp_version_info = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!IsValidNLSVersion func:{} locale:0x{:x} ver:0x{:x}",
        func,
        lp_locale_name,
        lp_version_info
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
