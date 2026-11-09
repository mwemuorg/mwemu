use crate::emu;

pub fn _wcslwr(emu: &mut emu::Emu, api: &str) {
    let s_ptr = emu.regs().rcx;
    let s = emu.maps.read_wide_string(s_ptr);
    let lower = s.to_lowercase();
    if s_ptr != 0 {
        emu.maps.write_wide_string(s_ptr, &lower);
    }
    log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, lower);
    emu.set_last_error(0);
    emu.regs_mut().rax = s_ptr;
}
