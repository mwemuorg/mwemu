use crate::emu;

pub(super) fn crt_ftol(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    log_red!(emu, "wincrt!_ftol {} => {}", val, val as i64);
    emu.regs_mut().rax = val as i64 as u64;
}
