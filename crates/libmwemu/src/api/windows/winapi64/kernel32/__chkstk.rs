use crate::emu;

pub fn __chkstk(emu: &mut emu::Emu) {
    let pages = emu.regs().rax;
    log_red!(emu, "kernel32!__chkstk pages:{}", pages);
    emu.set_last_error(0);
}
