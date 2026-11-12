use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strncmp(emu: &mut emu::Emu) {
    let s1_ptr = WinApiAbi::arg(emu, 0);
    let s2_ptr = WinApiAbi::arg(emu, 1);
    let n = WinApiAbi::arg(emu, 2) as usize;
    let s1 = emu.maps.read_string(s1_ptr);
    let s2 = emu.maps.read_string(s2_ptr);
    let cmp1: String = s1.chars().take(n).collect();
    let cmp2: String = s2.chars().take(n).collect();
    let result = cmp1.cmp(&cmp2) as i32;
    log_red!(emu, "msvcrt!strncmp '{}' '{}' {} => {}", s1, s2, n, result);

    emu.regs_mut().rax = result as u32 as u64;
}
