use crate::emu;

pub fn WSASetEvent(emu: &mut emu::Emu) {
    let event = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSASetEvent event:0x{:x}", event);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
