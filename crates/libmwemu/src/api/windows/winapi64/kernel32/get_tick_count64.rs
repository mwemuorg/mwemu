use crate::emu;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetTickCount64");
    emu.set_last_error(0);
    emu.regs_mut().rax = emu.tick as u64;
}
