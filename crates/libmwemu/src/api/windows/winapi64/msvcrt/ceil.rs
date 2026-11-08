use crate::emu;

pub fn ceil(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    let result = val.ceil();
    emu.fpu_mut().set_st(0, result);
    log_red!(emu, "msvcrt!ceil {} => {}", val, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
