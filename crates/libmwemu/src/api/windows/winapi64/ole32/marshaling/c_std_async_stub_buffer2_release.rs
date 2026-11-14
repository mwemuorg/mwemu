use crate::emu;
use crate::winapi::helper;

pub fn CStdAsyncStubBuffer2_Release(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "ole32!CStdAsyncStubBuffer2_Release arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    for _ in 0..2 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
