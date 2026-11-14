use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn atoi(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let s = emu.maps.read_string(str_ptr);
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!atoi '{}' => {}", s, val);

    emu.regs_mut().rax = val as u32 as u64;
}
