use crate::emu;

pub fn _ftol2(emu: &mut emu::Emu, api: &str) {
    let val = emu.fpu_mut().get_st(0);
    let result = val as i64;
    emu.fpu_mut().set_st(0, result as f64);
    log_red!(emu, "msvcrt!{} {} => {}", api, val, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
