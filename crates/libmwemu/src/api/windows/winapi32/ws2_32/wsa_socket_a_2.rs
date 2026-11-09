use crate::emu;
use crate::winapi::helper;

pub(super) fn WsaSocketA(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!WsaSocketA");

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::socket_create();
}
