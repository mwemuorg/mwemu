use crate::emu;
use crate::winapi::helper;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;

    log_red!(emu, "kernel32!CloseHandle 0x{:X}", handle);

    if helper::handler_exist(handle) {
        helper::handler_close(handle);
    }
    emu.handle_management.remove_file_handle(handle as u32);

    emu.regs_mut().rax = 1;
}
