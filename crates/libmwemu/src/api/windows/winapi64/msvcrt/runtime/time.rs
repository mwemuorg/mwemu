use crate::emu;

pub fn time(emu: &mut emu::Emu) {
    let time_ptr = emu.regs().rcx;
    let tick = emu.tick as u32;
    if time_ptr != 0 {
        emu.maps.write_dword(time_ptr, tick);
    }
    log_red!(emu, "msvcrt!time => {}", tick);
    emu.set_last_error(0);
    emu.regs_mut().rax = tick as u64;
}
