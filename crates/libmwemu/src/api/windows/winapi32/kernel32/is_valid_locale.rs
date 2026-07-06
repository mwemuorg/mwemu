use crate::emu;

pub fn IsValidLocale(emu: &mut emu::Emu) {
    let locale = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!IsValidLocale cannot read locale");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!IsValidLocale cannot read flags");

    log_red!(emu, "kernel32!IsValidLocale");

    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
