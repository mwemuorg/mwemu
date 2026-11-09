use crate::emu;

pub fn fmod(emu: &mut emu::Emu) {
    let x = f64::from_bits(emu.regs().rcx);
    let y = f64::from_bits(emu.regs().rdx);
    let result = x % y;
    log_red!(emu, "msvcrt!fmod {} {} => {}", x, y, result);
    emu.fpu_mut().set_st(0, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
