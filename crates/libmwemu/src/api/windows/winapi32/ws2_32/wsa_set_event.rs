use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSASetEvent(emu: &mut emu::Emu) {
    let event = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WSASetEvent event:0x{:x}", event);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
