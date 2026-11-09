use crate::emu;
use crate::winapi::helper;

pub fn DeregisterEventSource(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(hndl) {
        helper::handler_close(hndl);
    }
    log_red!(emu, "advapi32!DeregisterEventSource 0x{:x}", hndl);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
