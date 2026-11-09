use crate::emu;
use crate::winapi::helper;

pub fn DeleteService(emu: &mut emu::Emu) {
    let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(hsvc);
    log_red!(emu, "advapi32!DeleteService 0x{:x}", hsvc);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
