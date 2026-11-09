use crate::emu;
use crate::winapi::helper;

pub fn OleIsRunning(emu: &mut emu::Emu) {
    let pole_obj = emu.regs().rcx;
    log_red!(emu, "ole32!OleIsRunning pOleObj:0x{:x}", pole_obj);
    let running = if helper::handler_exist(pole_obj) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = running;
}
