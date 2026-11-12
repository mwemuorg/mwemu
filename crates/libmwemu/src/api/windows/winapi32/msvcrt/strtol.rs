use crate::emu;

pub fn strtol(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _endptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let base = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(10);
    let s = emu.maps.read_string(str_ptr);
    let val = i64::from_str_radix(s.trim(), base).unwrap_or(0);
    log_red!(emu, "msvcrt!strtol '{}' base:{} => {}", s, base, val);

    emu.regs_mut().rax = val as u64;
}
