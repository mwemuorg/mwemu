use crate::emu;

pub(super) fn crt_dllonexit(emu: &mut emu::Emu) {
    let func = emu.regs().rcx;
    log_red!(emu, "wincrt!__dllonexit 0x{:x}", func);
    emu.regs_mut().rax = func;
}
