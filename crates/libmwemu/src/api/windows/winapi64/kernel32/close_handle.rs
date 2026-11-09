use crate::emu;
use crate::winapi::common::{memory::Call, sections};
use crate::winapi::helper;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let section_handle = Call::Api.arg(emu, 0);
    log_red!(emu, "kernel32!CloseHandle 0x{:X}", section_handle);
    if sections::close(emu, section_handle) {
        Call::Api.finish(emu, 1, 1);
        return;
    }
    let handle = emu.regs().rcx;

    if helper::handler_exist(handle) {
        helper::handler_close(handle);
    }
    emu.handle_management.remove_file_handle(handle as u32);

    emu.regs_mut().rax = 1;
}
