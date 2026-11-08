use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;
pub fn DeactivateActCtxWorker(emu: &mut emu::Emu) {
    let _int = emu.regs().rcx as u32;
    let _ulCookie = emu.regs().rdx as u32;
    log_red!(emu, "kernel32!DeactivateActCtxWorker");
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
