use crate::emu;

pub fn lstrcmpW(emu: &mut emu::Emu) {
    let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    let s1 = emu.maps.read_wide_string(s1_ptr);
    let s2 = emu.maps.read_wide_string(s2_ptr);

    log_red!(emu, "kernel32!lstrcmpW '{}' == '{}'", s1, s2);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let result = s1.cmp(&s2);
    if result == std::cmp::Ordering::Less {
        emu.regs_mut().rax = 0xffffffff;
    } else if result == std::cmp::Ordering::Greater {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
