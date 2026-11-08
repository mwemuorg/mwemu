use crate::emu;

pub fn lstrcmpA(emu: &mut emu::Emu) {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;

    let s1 = emu.maps.read_string(s1_ptr);
    let s2 = emu.maps.read_string(s2_ptr);

    log_red!(emu, "kernel32!lstrcmp '{}' '{}'", s1, s2);
    emu.regs_mut().rax = s1.cmp(&s2) as u64;
}
