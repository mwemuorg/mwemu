use crate::emu;
use crate::winapi::helper;
pub fn WerUnregisterMemoryBlock(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!WerUnregisterMemoryBlock param1:0x{:x} param2:0x{:x}",
        param1,
        param2
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
