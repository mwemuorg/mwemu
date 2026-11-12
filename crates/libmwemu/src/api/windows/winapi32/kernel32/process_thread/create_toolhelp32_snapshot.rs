use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateToolhelp32Snapshot(emu: &mut emu::Emu) {
    let flags = WinApiAbi::arg(emu, 0) as u32;
    let pid = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!CreateToolhelp32Snapshot pid: {}", pid);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let uri = format!("pid://{}", pid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
