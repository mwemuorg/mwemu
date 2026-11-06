
use crate::emu;
use crate::winapi::helper;
pub fn DeleteSynchronizationBarrier(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!DeleteSynchronizationBarrier handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
