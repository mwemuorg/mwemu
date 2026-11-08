use crate::emu;

pub fn _time32(emu: &mut emu::Emu) {
    let timer_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let time_val = emu.tick as u32;
    if timer_ptr != 0 {
        emu.maps.write_dword(timer_ptr, time_val);
    }
    log_red!(emu, "msvcrt!_time32 => {}", time_val);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = time_val as u64;
}
