use crate::emu;

pub(super) fn abort(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!abort");
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
    emu.regs_mut().rax = 0;
}
