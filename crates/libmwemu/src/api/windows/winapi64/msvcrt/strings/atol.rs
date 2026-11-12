use crate::emu;

pub fn atol(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = emu.maps.read_string(str_ptr);
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!atol '{}' => {}", s, val);
    emu.set_last_error(0);
    emu.regs_mut().rax = val as u32 as u64;
}
