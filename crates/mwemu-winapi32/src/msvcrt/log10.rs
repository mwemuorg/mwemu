use crate::emu;

pub fn log10(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    let result = val.log10();
    emu.fpu_mut().set_st(0, result);
    log_red!(emu, "msvcrt!log10 {} => {}", val, result);
    emu.set_last_error(0);
}
