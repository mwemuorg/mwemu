use crate::emu;
use crate::winapi::helper;

pub fn WSASetBlockingHook(emu: &mut emu::Emu) {
    let hook_fn = emu.regs().rcx;
    log_red!(emu, "ws2_32!WSASetBlockingHook hook:0x{:x}", hook_fn);
    let handle = helper::handler_create("WSABlockingHook");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
