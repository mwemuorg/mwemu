use crate::emu;

pub fn LStrCmpIW(emu: &mut emu::Emu) {
    let sptr1 = emu.regs().rcx;
    let sptr2 = emu.regs().rdx;

    let s1 = emu.maps.read_wide_string(sptr1);
    let s2 = emu.maps.read_wide_string(sptr2);

    let s1_lower = s1.to_lowercase();
    let s2_lower = s2.to_lowercase();

    let result = match s1_lower.cmp(&s2_lower) {
        std::cmp::Ordering::Less => {
            log_red!(emu, "kernel32!lstrcmpiW `{}` < `{}`", s1, s2);
            -1i64 as u64
        }
        std::cmp::Ordering::Equal => {
            log_red!(emu, "kernel32!lstrcmpiW `{}` == `{}`", s1, s2);
            0
        }
        std::cmp::Ordering::Greater => {
            log_red!(emu, "kernel32!lstrcmpiW `{}` > `{}`", s1, s2);
            1
        }
    };

    emu.regs_mut().rax = result;
}
