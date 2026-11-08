use crate::emu;
use crate::winapi::helper;

pub fn WSASetEvent(emu: &mut emu::Emu) {
    let event = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSASetEvent event:0x{:x}", event);
    if helper::handler_exist(event) {
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0;
    }
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
