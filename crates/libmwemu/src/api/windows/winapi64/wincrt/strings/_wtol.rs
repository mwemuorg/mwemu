use crate::emu;

pub fn _wtol(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wincrt!_wtol '{}'", s);
    let val: i64 = s.trim().parse().unwrap_or(0);
    emu.set_last_error(0);
    emu.regs_mut().rax = val as u64;
}
