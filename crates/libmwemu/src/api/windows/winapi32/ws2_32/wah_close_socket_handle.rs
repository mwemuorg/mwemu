use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WahCloseSocketHandle(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WahCloseSocketHandle handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
