use crate::emu;

pub fn GetLargePageMinimum(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetLargePageMinimum => 0x200000 (2MB)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x200000;
}
