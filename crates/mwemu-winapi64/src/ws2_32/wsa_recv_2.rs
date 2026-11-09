use crate::emu;

pub(super) fn WsaRecv(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _buffers = emu.regs().rdx;
    let _count = emu.regs().r8;
    let bytes_recv_ptr = emu.regs().r9;
    log_red!(emu, "ws2_32!WSARecv sock:0x{:x}", sock);
    if bytes_recv_ptr != 0 {
        emu.maps.write_dword(bytes_recv_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
