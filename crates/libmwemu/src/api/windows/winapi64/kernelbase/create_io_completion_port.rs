use crate::emu;
use crate::winapi::helper;

pub fn CreateIoCompletionPort(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "kernelbase!CreateIoCompletionPort arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    let hndl = helper::handler_create("iocp://CreateIoCompletionPort");
    emu.regs_mut().rax = hndl;
}
