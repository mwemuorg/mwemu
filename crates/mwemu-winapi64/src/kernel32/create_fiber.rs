use crate::emu;
use crate::winapi::helper;

pub fn CreateFiber(emu: &mut emu::Emu) {
    let stack_size = emu.regs().rcx;
    let start_addr = emu.regs().rdx;
    let parameter = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!CreateFiber stack_size: 0x{:x} start: 0x{:x} param: 0x{:x}",
        stack_size,
        start_addr,
        parameter
    );

    emu.regs_mut().rax = helper::handler_create("CreateFiber");
}
