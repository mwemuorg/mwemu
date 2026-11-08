use crate::emu;

pub fn _time64(emu: &mut emu::Emu) {
    let t_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let now = 0x6718_0000u64;
    if t_ptr != 0 {
        emu.maps.write_qword(t_ptr, now);
    }
    log_red!(emu, "msvcrt!_time64 => 0x{:x}", now);
    emu.regs_mut().rax = now;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
