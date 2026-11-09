use crate::emu;
use crate::winapi::helper;

pub fn WSApSetPostRoutine(emu: &mut emu::Emu) {
    let routine_ptr = emu.regs().rcx;
    log_red!(emu, "ws2_32!WSApSetPostRoutine routine:0x{:x}", routine_ptr);
    if routine_ptr != 0 {
        let _ = helper::handler_create("WSAPostRoutine");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
