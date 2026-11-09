use crate::emu;
use crate::winapi::helper;

pub fn WmiOpenBlock(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "advapi32!WmiOpenBlock 0x{:x}", arg1);
    let handle = helper::handler_create("WmiOpenBlock");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
