use crate::emu;

pub fn _strlwr(emu: &mut emu::Emu, api: &str) {
    let s_ptr = emu.regs().rcx;
    let s = emu.maps.read_string(s_ptr);
    let lower = s.to_ascii_lowercase();
    if s_ptr != 0 {
        emu.maps.write_string(s_ptr, &lower);
    }
    log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, lower);
    emu.set_last_error(0);
    emu.regs_mut().rax = s_ptr;
}
