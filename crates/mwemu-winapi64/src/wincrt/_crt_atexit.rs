use crate::emu;

pub(super) fn _crt_atexit(emu: &mut emu::Emu) {
    let function = emu.regs().rcx;
    log_red!(emu, "wincrt!_crt_atexit function: 0x{:x}", function);
    emu.regs_mut().rax = 0;
}
