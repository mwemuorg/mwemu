use crate::emu;
use crate::winapi::helper;

pub fn CancelIoEx(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx;
    let overlapped = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!CancelIoEx h:0x{:x} overlapped:0x{:x}",
        file_handle,
        overlapped
    );
    if helper::handler_exist(file_handle) {
        emu.regs_mut().rax = 1; // TRUE
    } else {
        emu.regs_mut().rax = 0; // FALSE
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
    }
}
