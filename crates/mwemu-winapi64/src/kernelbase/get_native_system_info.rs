use crate::emu;

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetNativeSystemInfo ptr:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        emu.maps.write_word(p0, 9);
        emu.maps.write_word(p0 + 2, 0);
        emu.maps.write_dword(p0 + 4, 4096);
        emu.maps.write_dword(p0 + 20, 4);
        emu.maps.write_dword(p0 + 24, 6);
        emu.maps.write_word(p0 + 28, 0x8664);
    }
    emu.set_last_error(0);
}
