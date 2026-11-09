use crate::emu;
use crate::winapi::helper;

pub fn CreateRemoteThreadEx(emu: &mut emu::Emu) {
    let hprocess = emu.regs().rcx;
    let thread_attr = emu.regs().rdx;
    let stack_size = emu.regs().r8;
    let start_addr = emu.regs().r9;
    let parameter = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let creation_flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let attr_list = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

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
}
