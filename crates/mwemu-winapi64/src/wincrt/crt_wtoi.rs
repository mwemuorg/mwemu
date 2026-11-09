use crate::emu;

pub(super) fn crt_wtoi(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let ws = emu.maps.read_wide_string(ptr);
    let val = ws.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "wincrt!_wtoi '{}' => {}", ws, val);
    emu.regs_mut().rax = val as u64;
}
