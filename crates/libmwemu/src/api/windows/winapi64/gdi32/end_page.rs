use crate::emu;

pub fn EndPage(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!EndPage hdc:0x{:x}", hdc);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
