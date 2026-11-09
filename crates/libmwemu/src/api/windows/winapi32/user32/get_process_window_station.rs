use crate::emu;

pub(super) fn GetProcessWindowStation(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetProcessWindowStation");

    emu.regs_mut().rax = 0x1337; // get handler
}
