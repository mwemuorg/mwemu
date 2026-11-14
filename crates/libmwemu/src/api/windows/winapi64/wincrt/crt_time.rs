use crate::emu;

pub(super) fn crt_time(emu: &mut emu::Emu) {
    let t_ptr = emu.regs().rcx;
    let now = 0x6718_0000u64;
    if t_ptr != 0 {
        emu.maps.write_qword(t_ptr, now);
    }
    log_red!(emu, "wincrt!time => 0x{:x}", now);
    emu.regs_mut().rax = now;
}
