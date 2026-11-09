use crate::emu;

pub fn WSAGetQOSByName(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let qos_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSAGetQOSByName sock:0x{:x}", sock);
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
