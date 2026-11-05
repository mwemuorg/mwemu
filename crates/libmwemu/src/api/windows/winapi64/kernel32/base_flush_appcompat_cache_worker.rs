use crate::emu;

pub fn BaseFlushAppcompatCacheWorker(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BaseFlushAppcompatCacheWorker");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
