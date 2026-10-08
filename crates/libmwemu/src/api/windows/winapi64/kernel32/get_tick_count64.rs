use crate::emu;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetTickCount64");
    emu.regs_mut().rax = emu.tick as u64;
}
