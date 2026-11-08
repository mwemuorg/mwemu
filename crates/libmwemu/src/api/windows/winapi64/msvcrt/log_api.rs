use crate::emu;

pub fn log(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    let result = val.ln();
    emu.fpu_mut().set_st(0, result);
    log_red!(emu, "msvcrt!log {} => {}", val, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
