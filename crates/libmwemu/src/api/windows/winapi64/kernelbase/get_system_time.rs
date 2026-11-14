use crate::emu;

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetSystemTime ptr:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        emu.maps.write_word(p0, 2024);
        emu.maps.write_word(p0 + 2, 1);
        emu.maps.write_word(p0 + 4, 3);
        emu.maps.write_word(p0 + 6, 15);
        emu.maps.write_word(p0 + 8, 10);
        emu.maps.write_word(p0 + 10, 30);
        emu.maps.write_word(p0 + 12, 0);
        emu.maps.write_word(p0 + 14, 0);
    }
    emu.set_last_error(0);
}
