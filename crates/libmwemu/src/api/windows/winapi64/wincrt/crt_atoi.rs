use crate::emu;

pub(super) fn crt_atoi(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let s = emu.maps.read_string(ptr);
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "wincrt!atoi '{}' => {}", s, val);
    emu.regs_mut().rax = val as u64;
}
