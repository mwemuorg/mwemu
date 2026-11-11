use crate::emu;

pub fn AddFontResourceExA(emu: &mut emu::Emu) {
    let name = emu.maps.read_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(emu, "gdi32!AddFontResourceExA '{}' flags:0x{:x}", name, fl);
    emu.regs_mut().rax = 1;
}

pub fn AddFontResourceExW(emu: &mut emu::Emu) {
    let name = emu.maps.read_wide_string(emu.regs().rcx);
    let fl = emu.regs().rdx;
    log_red!(emu, "gdi32!AddFontResourceExW '{}' flags:0x{:x}", name, fl);
    emu.regs_mut().rax = 1;
}
