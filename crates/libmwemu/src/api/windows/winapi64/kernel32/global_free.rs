use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::helper;
pub fn GlobalFree(emu: &mut emu::Emu) {
    let mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalFree mem: 0x{:x}", mem);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    heap_engine::heap_free(emu, 0, mem);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
