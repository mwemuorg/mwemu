use crate::emu;
use crate::windows::constants;

pub fn DeactivateActCtxWorker(emu: &mut emu::Emu) {
    let _int = emu.regs().rcx as u32;
    let _ulCookie = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!DeactivateActCtxWorker");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
