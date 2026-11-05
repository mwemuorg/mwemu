use crate::emu;

pub fn lstrcmpW(emu: &mut emu::Emu) {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;

    let s1 = emu.maps.read_wide_string(s1_ptr);
    let s2 = emu.maps.read_wide_string(s2_ptr);

    log_red!(emu, "kernel32!lstrcmpW '{}' == '{}'", s1, s2);

    let result = s1.cmp(&s2);
    if result == std::cmp::Ordering::Less {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0xffffffff;
    } else if result == std::cmp::Ordering::Greater {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
