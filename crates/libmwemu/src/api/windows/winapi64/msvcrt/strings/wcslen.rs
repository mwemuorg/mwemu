use crate::emu;

pub fn wcslen(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;
    let s = emu.maps.read_wide_string(s_ptr);
    let len = s.len();
    log_red!(emu, "msvcrt!wcslen '{}' => {}", s, len);
    emu.set_last_error(0);
    emu.regs_mut().rax = len as u64;
}
