use crate::emu;

pub(super) fn crt_exit(emu: &mut emu::Emu, api: &str) {
    let code = emu.regs().rcx;
    log_red!(emu, "wincrt!{} code:{}", api, code);
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
    emu.regs_mut().rax = 0;
}
