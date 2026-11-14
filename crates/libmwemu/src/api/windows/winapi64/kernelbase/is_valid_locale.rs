use crate::emu;

pub fn IsValidLocale(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let flags = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!IsValidLocale locale:0x{:x} flags:0x{:x}",
        locale,
        flags
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
