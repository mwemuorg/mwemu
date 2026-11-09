use crate::emu;
use crate::winapi::helper;

pub fn WSAGetLastError(emu: &mut emu::Emu) {
    let err = emu.get_last_error();
    log_red!(emu, "ws2_32!WSAGetLastError = {}", err);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = err;
}
