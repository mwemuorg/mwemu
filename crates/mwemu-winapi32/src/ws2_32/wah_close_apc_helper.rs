use crate::emu;
use crate::winapi::helper;

pub fn WahCloseApcHelper(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WahCloseApcHelper handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
