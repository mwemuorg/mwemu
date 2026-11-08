use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateRemoteThreadEx(emu: &mut emu::Emu) {
    let hprocess = arg(emu, 0) as u64;
    let thread_attr = arg(emu, 1) as u64;
    let stack_size = arg(emu, 2) as u64;
    let start_addr = arg(emu, 3) as u64;
    let parameter = arg(emu, 4) as u64;
    let creation_flags = arg(emu, 5) as u64;
    let attr_list = arg(emu, 6) as u64;

    log_red!(
        emu,
        "kernel32!CreateRemoteThreadEx hProcess: 0x{:x} attr: 0x{:x} stack: 0x{:x} start: 0x{:x} param: 0x{:x} flags: 0x{:x} attr_list: 0x{:x}",
        hprocess,
        thread_attr,
        stack_size,
        start_addr,
        parameter,
        creation_flags,
        attr_list
    );

    emu.regs_mut().rax = helper::handler_create("CreateRemoteThreadEx");
    stdcall_ret(emu, 7);
}
