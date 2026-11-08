use crate::emu;
use crate::winapi::helper;

pub fn WSASetLastError(emu: &mut emu::Emu) {
    let error_code = emu.regs().rcx as u32;
    helper::advance_tick(emu, 1);
    emu.set_last_error(error_code as u64);
    log_red!(emu, "ws2_32!WSASetLastError error: {}", error_code);
}
