use crate::emu;

pub fn time(emu: &mut emu::Emu) {
    let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let tick = emu.tick as u32;
    if time_ptr != 0 {
        emu.maps.write_dword(time_ptr, tick);
    }
    log_red!(emu, "msvcrt!time => {}", tick);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = tick as u64;
}
