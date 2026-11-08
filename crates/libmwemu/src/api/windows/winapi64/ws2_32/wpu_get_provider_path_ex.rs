use crate::emu;

pub fn WPUGetProviderPathEx(emu: &mut emu::Emu) {
    let provider_id_ptr = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    let path_len_ptr = emu.regs().r8;
    let flags = emu.regs().r9;
    log_red!(emu, "ws2_32!WPUGetProviderPathEx flags:{}", flags);
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
