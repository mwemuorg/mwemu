use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateFiberEx(emu: &mut emu::Emu) {
    let commit_size = arg(emu, 0) as u64;
    let reserve_size = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let start_addr = arg(emu, 3) as u64;
    let parameter = arg(emu, 4) as u64;

    log_red!(
        emu,
        "kernel32!CreateFiberEx commit: 0x{:x} reserve: 0x{:x} flags: 0x{:x} start: 0x{:x} param: 0x{:x}",
        commit_size,
        reserve_size,
        flags,
        start_addr,
        parameter
    );

    emu.regs_mut().rax = helper::handler_create("CreateFiberEx");
    stdcall_ret(emu, 5);
}
