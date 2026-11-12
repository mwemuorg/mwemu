use crate::emu;

pub fn _atoi64(emu: &mut emu::Emu, api: &str) {
    let str_ptr = emu.regs().rcx;
    let s = if api == "_wtoi64" {
        emu.maps.read_wide_string(str_ptr)
    } else {
        emu.maps.read_string(str_ptr)
    };
    let val = s.trim().parse::<i64>().unwrap_or(0);
    log_red!(emu, "msvcrt!{} '{}' => {}", api, s, val);
    emu.set_last_error(0);
    emu.regs_mut().rax = val as u64;
}
