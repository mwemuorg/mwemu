use crate::emu;

pub(super) fn crt_endthread(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
}
