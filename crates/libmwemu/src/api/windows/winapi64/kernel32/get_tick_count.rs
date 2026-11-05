use crate::emu;

pub fn GetTickCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetTickCount");
    // TODO: increment the tick?
    emu.set_last_error(0);
    emu.regs_mut().rax = emu.tick as u64;
}
