use crate::emu;
use crate::winapi::helper;

pub fn OpenProcess(emu: &mut emu::Emu) {
    let access = emu.regs().rcx;
    let inherit = emu.regs().rdx;
    let pid = emu.regs().r8;

    log_red!(emu, "kernel32!OpenProcess pid: {}", pid);

    let uri = format!("pid://{}", pid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
