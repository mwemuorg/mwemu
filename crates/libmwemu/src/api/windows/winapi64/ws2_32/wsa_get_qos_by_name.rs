use crate::emu;

pub fn WSAGetQOSByName(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let qos_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSAGetQOSByName sock:0x{:x}", sock);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
