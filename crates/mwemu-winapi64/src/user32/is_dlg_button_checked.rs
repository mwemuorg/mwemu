use crate::emu;

pub fn IsDlgButtonChecked(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    log_red!(emu, "user32!IsDlgButtonChecked p1:0x{:x}", p1);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
