use crate::emu;

pub fn GetTickCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetTickCount");

    emu.regs_mut().rax = emu.tick as u64;
}
