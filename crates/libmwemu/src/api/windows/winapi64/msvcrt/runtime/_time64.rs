use crate::emu;

pub fn _time64(emu: &mut emu::Emu) {
    let t_ptr = emu.regs().rcx;
    let now = 0x6718_0000u64;
    if t_ptr != 0 {
        emu.maps.write_qword(t_ptr, now);
    }
    log_red!(emu, "msvcrt!_time64 => 0x{:x}", now);
    emu.set_last_error(0);
    emu.regs_mut().rax = now;
}
