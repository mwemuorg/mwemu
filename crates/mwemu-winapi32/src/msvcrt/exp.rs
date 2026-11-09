use crate::emu;

pub fn exp(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    let result = val.exp();
    emu.fpu_mut().set_st(0, result);
    log_red!(emu, "msvcrt!exp {} => {}", val, result);
    emu.set_last_error(0);
}
