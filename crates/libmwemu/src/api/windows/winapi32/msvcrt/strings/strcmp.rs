use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strcmp(emu: &mut emu::Emu) {
    let s1_ptr = WinApiAbi::arg(emu, 0);
    let s2_ptr = WinApiAbi::arg(emu, 1);
    let s1 = emu.maps.read_string(s1_ptr);
    let s2 = emu.maps.read_string(s2_ptr);
    let result = s1.cmp(&s2) as i32;
    log_red!(emu, "msvcrt!strcmp '{}' '{}' => {}", s1, s2, result);

    emu.regs_mut().rax = result as u32 as u64;
}
