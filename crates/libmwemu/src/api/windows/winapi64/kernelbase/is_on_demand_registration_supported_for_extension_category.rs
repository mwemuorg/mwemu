use crate::emu;

pub fn IsOnDemandRegistrationSupportedForExtensionCategory(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernelbase!IsOnDemandRegistrationSupportedForExtensionCategory 0x{:x} => TRUE",
        arg1
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
