use crate::emu;

pub fn wcslen(emu: &mut emu::Emu) {
    let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(s_ptr);
    let len = s.len();
    log_red!(emu, "msvcrt!wcslen '{}' => {}", s, len);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = len as u64;
}
