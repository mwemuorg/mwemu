use crate::emu;

pub(super) fn crt_cxx_throw(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_CxxThrowException");
    emu.regs_mut().rax = 0;
}
