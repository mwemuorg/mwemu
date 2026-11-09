use crate::emu;

pub(super) fn crt_onexit(emu: &mut emu::Emu) {
    let func = emu.regs().rcx;
    log_red!(emu, "wincrt!_onexit 0x{:x}", func);
    emu.regs_mut().rax = func;
}
