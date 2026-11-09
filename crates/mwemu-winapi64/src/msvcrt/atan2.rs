use crate::emu;

pub fn atan2(emu: &mut emu::Emu) {
    let y = f64::from_bits(emu.regs().rcx);
    let x = f64::from_bits(emu.regs().rdx);
    let result = y.atan2(x);
    log_red!(emu, "msvcrt!atan2 {} {} => {}", y, x, result);
    emu.fpu_mut().set_st(0, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
