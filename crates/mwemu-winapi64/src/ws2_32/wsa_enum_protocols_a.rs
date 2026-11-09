use crate::emu;

pub fn WSAEnumProtocolsA(emu: &mut emu::Emu) {
    let protocols_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let buf_len_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSAEnumProtocolsA");
    if buf_len_ptr != 0 {
        emu.maps.write_dword(buf_len_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
