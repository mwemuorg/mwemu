use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateFiber(emu: &mut emu::Emu) {
    let stack_size = arg(emu, 0) as u64;
    let start_addr = arg(emu, 1) as u64;
    let parameter = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!CreateFiber stack_size: 0x{:x} start: 0x{:x} param: 0x{:x}",
        stack_size,
        start_addr,
        parameter
    );

    emu.regs_mut().rax = helper::handler_create("CreateFiber");
    stdcall_ret(emu, 3);
}
