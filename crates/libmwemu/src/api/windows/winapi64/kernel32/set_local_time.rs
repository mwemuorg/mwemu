use crate::emu;

pub fn SetLocalTime(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernel32!SetLocalTime");
    if emu.regs().rcx != 0 && emu.maps.is_mapped(emu.regs().rcx) {
        let _ = emu.maps.read_word(emu.regs().rcx).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
