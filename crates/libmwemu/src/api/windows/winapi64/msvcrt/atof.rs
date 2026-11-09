use crate::emu;

pub fn atof(emu: &mut emu::Emu, api: &str) {
    let str_ptr = emu.regs().rcx;
    let s = emu.maps.read_string(str_ptr);
    let val = s.trim().parse::<f64>().unwrap_or(0.0);
    log_red!(emu, "msvcrt!{} '{}' => {}", api, s, val);
    emu.fpu_mut().set_st(0, val);
    emu.set_last_error(0);
    emu.regs_mut().rax = val.to_bits();
}
