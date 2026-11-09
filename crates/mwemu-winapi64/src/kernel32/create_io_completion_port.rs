use crate::emu;
use crate::winapi::helper;

pub fn CreateIoCompletionPort(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx;
    let existing_port = emu.regs().rdx;
    let completion_key = emu.regs().r8;
    let num_concurrent = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!CreateIoCompletionPort file_handle: 0x{:x} existing_port: 0x{:x} key: 0x{:x} concurrent: {}",
        file_handle,
        existing_port,
        completion_key,
        num_concurrent
    );

    emu.regs_mut().rax = helper::handler_create("CreateIoCompletionPort");
}
