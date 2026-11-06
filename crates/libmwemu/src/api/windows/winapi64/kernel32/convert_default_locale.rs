use crate::emu;

pub fn ConvertDefaultLocale(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;

    log_red!(emu, "kernel32!ConvertDefaultLocale locale:0x{:x}", locale);
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }

    let result = match locale {
        0x0800 => 0x0409,
        0x0400 => 0x0409,
        0x0C00 => 0x0409,
        other => other,
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
