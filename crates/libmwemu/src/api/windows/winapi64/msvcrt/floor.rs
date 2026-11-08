use crate::emu;

pub fn floor(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    let result = val.floor();
    emu.fpu_mut().set_st(0, result);
    log_red!(emu, "msvcrt!floor {} => {}", val, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
