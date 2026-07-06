use crate::emu;
use crate::winapi::helper;

pub fn CreateToolhelp32Snapshot(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let pid = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!CreateToolhelp32Snapshot flags: {:x} pid: {}",
        flags,
        pid
    );

    let uri = format!("CreateToolhelp32Snapshot://{}", pid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
