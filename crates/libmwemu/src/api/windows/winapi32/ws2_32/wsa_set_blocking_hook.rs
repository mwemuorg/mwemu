use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSASetBlockingHook(emu: &mut emu::Emu) {
    let hook_fn = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WSASetBlockingHook hook:0x{:x}", hook_fn);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
