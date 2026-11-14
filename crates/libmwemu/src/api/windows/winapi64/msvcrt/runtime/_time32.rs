use crate::emu;

pub fn _time32(emu: &mut emu::Emu) {
    let timer_ptr = emu.regs().rcx;
    let time_val = emu.tick as u32;
    if timer_ptr != 0 {
        emu.maps.write_dword(timer_ptr, time_val);
    }
    log_red!(emu, "msvcrt!_time32 => {}", time_val);
    emu.set_last_error(0);
    emu.regs_mut().rax = time_val as u64;
}
