use crate::emu;

pub fn lstrlenW(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;

    let s = emu.maps.read_wide_string(s_ptr);
    let len = s.chars().count() as u64;

    log_red!(emu, "kernel32!lstrlenW '{}' ={}", s, len);

    emu.set_last_error(0);
    emu.regs_mut().rax = len;
}
