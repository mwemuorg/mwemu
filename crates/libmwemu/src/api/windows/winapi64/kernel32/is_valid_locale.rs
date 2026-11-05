use crate::emu;

pub fn IsValidLocale(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx as u32;
    let flags = emu.regs().rdx;
    let valid = matches!(
        locale,
        0x0409
            | 0x0000
            | 0x0800
            | 0x0c09
            | 0x0809
            | 0x0407
            | 0x040c
            | 0x0410
            | 0x0411
            | 0x0412
            | 0x0804
            | 0x0c0a
            | 0x0416
            | 0x0419
            | 0x041f
    );
    log_red!(
        emu,
        "kernel32!IsValidLocale locale:0x{:x} flags:0x{:x} => {}",
        locale,
        flags,
        valid
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if valid { 1 } else { 0 };
}
