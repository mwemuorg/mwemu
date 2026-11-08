use crate::emu;

pub(super) fn puts(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let msg = emu.maps.read_string(s);
    log_red!(emu, "wincrt!puts '{}'", msg);
    emu.regs_mut().rax = 0;
}
