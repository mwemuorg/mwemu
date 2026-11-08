use crate::emu;

pub fn WahWriteLSPEvent(emu: &mut emu::Emu) {
    let event = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WahWriteLSPEvent event:0x{:x}", event);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
