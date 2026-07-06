use crate::emu;
use crate::winapi::helper;

pub fn CreateToolhelp32Snapshot(emu: &mut emu::Emu) {
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateToolhelp32Snapshot cannot read flags");
    let pid = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CreateToolhelp32Snapshot cannot read pid");

    log_red!(emu, "kernel32!CreateToolhelp32Snapshot pid: {}", pid);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let uri = format!("pid://{}", pid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
