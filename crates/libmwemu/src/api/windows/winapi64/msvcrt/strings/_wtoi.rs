use crate::emu;

pub fn _wtoi(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let ws = emu.maps.read_wide_string(ptr);
    let val = ws.parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!_wtoi '{}' => {}", ws, val);
    emu.set_last_error(0);
    emu.regs_mut().rax = val as u64;
}
