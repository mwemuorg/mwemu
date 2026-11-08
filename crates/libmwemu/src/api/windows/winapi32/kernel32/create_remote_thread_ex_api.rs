use crate::emu;
use crate::winapi::helper;

pub fn CreateRemoteThreadEx_api(emu: &mut emu::Emu) {
    for _ in 0..8u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateRemoteThreadEx");
    emu.regs_mut().rax = helper::handler_create("CreateRemoteThreadEx");
}
