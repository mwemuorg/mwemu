use crate::emu;

pub fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    let name = emu.maps.read_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!RemoveFontResourceExA '{}' flags:0x{:x}",
        name,
        fl
    );
    emu.regs_mut().rax = 1;
}

pub fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    let name = emu.maps.read_wide_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!RemoveFontResourceExW '{}' flags:0x{:x}",
        name,
        fl
    );
    emu.regs_mut().rax = 1;
}
