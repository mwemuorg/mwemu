use crate::emu;
use crate::winapi::helper;

pub fn CloseServiceHandle(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(handle);
    log_red!(emu, "advapi32!CloseServiceHandle 0x{:x}", handle);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
