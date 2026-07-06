use crate::emu;

pub fn GetLogicalDrives(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetLogicalDrives");

    emu.regs_mut().rax = 0xc;
}
