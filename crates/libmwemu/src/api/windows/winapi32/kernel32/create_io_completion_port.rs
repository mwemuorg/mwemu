use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateIoCompletionPort(emu: &mut emu::Emu) {
    let file_handle = arg(emu, 0) as u64;
    let existing_port = arg(emu, 1) as u64;
    let completion_key = arg(emu, 2) as u64;
    let num_concurrent = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!CreateIoCompletionPort file_handle: 0x{:x} existing_port: 0x{:x} key: 0x{:x} concurrent: {}",
        file_handle,
        existing_port,
        completion_key,
        num_concurrent
    );

    emu.regs_mut().rax = helper::handler_create("CreateIoCompletionPort");
    stdcall_ret(emu, 4);
}
