use crate::emu;
use crate::winapi::helper;

pub fn OpenThreadWaitChainSession(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let hndl = helper::handler_create("OpenThreadWaitChainSession");
    log_red!(emu, "advapi32!OpenThreadWaitChainSession => 0x{:x}", hndl);
    emu.regs_mut().rax = hndl;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
