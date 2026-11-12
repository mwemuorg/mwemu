use crate::emu;
use crate::winapi::common::strings;

pub fn lstrcmpA(emu: &mut emu::Emu) {
    lstrcmp(emu, "lstrcmpA", false);
}

fn lstrcmp(emu: &mut emu::Emu, api: &str, wide: bool) {
    let s1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    let s1 = strings::read_text(emu, s1_ptr, wide);
    let s2 = strings::read_text(emu, s2_ptr, wide);

    log_red!(emu, "kernel32!{api} '{}' == '{}'", s1, s2);

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

pub fn lstrcmpW(emu: &mut emu::Emu) {
    lstrcmp(emu, "lstrcmpW", true);
}
