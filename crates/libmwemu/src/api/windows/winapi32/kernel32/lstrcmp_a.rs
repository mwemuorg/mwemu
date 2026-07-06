use crate::emu;

pub fn lstrcmpA(emu: &mut emu::Emu) {
    let s1_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!lstrcmp cannot read s1_ptr") as u64;
    let s2_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!lstrcmp cannot read s2_ptr") as u64;

    let s1 = emu.maps.read_string(s1_ptr);
    let s2 = emu.maps.read_string(s2_ptr);

    log_red!(emu, "kernel32!lstrcmpA '{}' == '{}'", s1, s2);

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
