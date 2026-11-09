use crate::emu;
use crate::winapi::helper;

pub fn CreateFiberEx(emu: &mut emu::Emu) {
    let commit_size = emu.regs().rcx;
    let reserve_size = emu.regs().rdx;
    let flags = emu.regs().r8;
    let start_addr = emu.regs().r9;
    let parameter = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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
}
