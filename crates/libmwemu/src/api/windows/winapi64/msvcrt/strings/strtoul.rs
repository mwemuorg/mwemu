use crate::emu;

pub fn strtoul(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let _endptr = emu.regs().rdx;
    let base = emu.regs().r8 as u32;
    let s = emu.maps.read_string(str_ptr);
    let val = u64::from_str_radix(s.trim(), base).unwrap_or(0) as u32;
    log_red!(emu, "msvcrt!strtoul '{}' base:{} => {}", s, base, val);
    emu.set_last_error(0);
    emu.regs_mut().rax = val as u64;
}
