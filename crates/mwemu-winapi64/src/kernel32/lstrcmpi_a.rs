use crate::emu;

pub fn lstrcmpiA(emu: &mut emu::Emu) {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;

    let s1 = emu.maps.read_string(s1_ptr);
    let s2 = emu.maps.read_string(s2_ptr);

    let result = match s1.to_lowercase().cmp(&s2.to_lowercase()) {
        std::cmp::Ordering::Less => {
            log_red!(emu, "kernel32!lstrcmpiA '{}' < '{}'", s1, s2);
            -1i64 as u64
        }
        std::cmp::Ordering::Equal => {
            log_red!(emu, "kernel32!lstrcmpiA '{}' == '{}'", s1, s2);
            0
        }
        std::cmp::Ordering::Greater => {
            log_red!(emu, "kernel32!lstrcmpiA '{}' > '{}'", s1, s2);
            1
        }
    };

    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
