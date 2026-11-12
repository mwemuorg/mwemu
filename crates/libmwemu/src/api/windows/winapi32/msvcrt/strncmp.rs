use crate::emu;

pub fn strncmp(emu: &mut emu::Emu) {
    let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let s1 = emu.maps.read_string(s1_ptr);
    let s2 = emu.maps.read_string(s2_ptr);
    let cmp1: String = s1.chars().take(n).collect();
    let cmp2: String = s2.chars().take(n).collect();
    let result = cmp1.cmp(&cmp2) as i32;
    log_red!(emu, "msvcrt!strncmp '{}' '{}' {} => {}", s1, s2, n, result);

    emu.regs_mut().rax = result as u32 as u64;
}
