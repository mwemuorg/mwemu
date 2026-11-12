use crate::emu;

pub fn wcsncmp(emu: &mut emu::Emu) {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let s1 = emu.maps.read_wide_string(s1_ptr);
    let s2 = emu.maps.read_wide_string(s2_ptr);
    let cmp1: String = s1.chars().take(n).collect();
    let cmp2: String = s2.chars().take(n).collect();
    let result = cmp1.cmp(&cmp2) as i32;
    log_red!(emu, "msvcrt!wcsncmp '{}' '{}' {} => {}", s1, s2, n, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u32 as u64;
}
