use crate::emu;

pub fn _wcsupr(emu: &mut emu::Emu, api: &str) {
    let s_ptr = emu.regs().rcx;
    let s = emu.maps.read_wide_string(s_ptr);
    let upper = s.to_uppercase();
    if s_ptr != 0 {
        emu.maps.write_wide_string(s_ptr, &upper);
    }
    log_red!(emu, "msvcrt!{} '{}' => '{}'", api, s, upper);
    emu.set_last_error(0);
    emu.regs_mut().rax = s_ptr;
}
