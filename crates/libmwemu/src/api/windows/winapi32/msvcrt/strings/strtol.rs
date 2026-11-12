use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strtol(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let _endptr = WinApiAbi::arg(emu, 1) as u32;
    let base = WinApiAbi::try_arg(emu, 2).unwrap_or(10) as u32;
    let s = emu.maps.read_string(str_ptr);
    let val = i64::from_str_radix(s.trim(), base).unwrap_or(0);
    log_red!(emu, "msvcrt!strtol '{}' base:{} => {}", s, base, val);

    emu.regs_mut().rax = val as u64;
}
