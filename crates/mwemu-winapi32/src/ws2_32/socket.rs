use crate::emu;
use crate::winapi::helper;

pub fn socket(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!socket");

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::socket_create();
}
