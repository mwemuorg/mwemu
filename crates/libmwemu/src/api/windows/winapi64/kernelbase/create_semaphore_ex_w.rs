use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateSemaphoreExW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    log_red!(
        emu,
        "kernelbase!CreateSemaphoreExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    );
    let hndl = helper::handler_create("semaphore://CreateSemaphoreExW");
    emu.regs_mut().rax = hndl;
}
