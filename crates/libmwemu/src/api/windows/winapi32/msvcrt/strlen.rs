use crate::emu;

pub fn strlen(emu: &mut emu::Emu) {
    let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = emu.maps.read_string(s_ptr);
    let len = s.len();
    log_red!(emu, "msvcrt!strlen '{}' => {}", s, len);

    emu.regs_mut().rax = len as u64;
}
