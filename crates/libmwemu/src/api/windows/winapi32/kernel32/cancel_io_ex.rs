use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CancelIoEx(emu: &mut emu::Emu) {
    let file_handle = arg(emu, 0) as u64;
    let overlapped = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}
