use crate::emu;
use crate::winapi::helper;

pub fn OleLockRunning(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(emu, "ole32!OleLockRunning 0x{:x}", arg1);
    let handle = helper::handler_create("OleLockRunning");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
