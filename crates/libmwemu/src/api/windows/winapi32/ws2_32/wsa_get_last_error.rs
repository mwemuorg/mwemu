use crate::emu;

pub(super) fn WsaGetLastError(emu: &mut emu::Emu) {
    let err = emu.get_last_error();
    log_red!(emu, "ws2_32!WSAGetLastError => {}", err);
    emu.regs_mut().rax = err;
}
