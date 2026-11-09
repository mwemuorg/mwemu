use crate::emu;

pub fn pow(emu: &mut emu::Emu) {
    let base_val = f64::from_bits(emu.regs().rcx);
    let exp_val = f64::from_bits(emu.regs().rdx);
    let result = base_val.powf(exp_val);
    log_red!(emu, "msvcrt!pow {} {} => {}", base_val, exp_val, result);
    emu.fpu_mut().set_st(0, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
