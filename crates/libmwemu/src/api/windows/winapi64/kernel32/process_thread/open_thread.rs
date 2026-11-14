use crate::emu;
use crate::winapi::helper;

pub fn OpenThread(emu: &mut emu::Emu) {
    let access = emu.regs().rcx;
    let inherit = emu.regs().rdx;
    let tid = emu.regs().r8;

    log_red!(emu, "kernel32!OpenThread tid: {}", tid);

    let uri = format!("tid://{}", tid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
