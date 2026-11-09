use crate::emu;

pub fn cos(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    let result = val.cos();
    emu.fpu_mut().set_st(0, result);
    log_red!(emu, "msvcrt!cos {} => {}", val, result);
    emu.set_last_error(0);
}
