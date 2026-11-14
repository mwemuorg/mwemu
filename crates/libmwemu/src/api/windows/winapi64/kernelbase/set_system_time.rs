use crate::emu;

pub fn SetSystemTime(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernelbase!SetSystemTime ptr:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        let _year = emu.maps.read_word(p0).unwrap_or(0);
        let _month = emu.maps.read_word(p0 + 2).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
