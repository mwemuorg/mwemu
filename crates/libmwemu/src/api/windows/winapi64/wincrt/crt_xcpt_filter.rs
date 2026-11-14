use crate::emu;

pub(super) fn crt_xcpt_filter(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
    emu.regs_mut().rax = 0;
}
