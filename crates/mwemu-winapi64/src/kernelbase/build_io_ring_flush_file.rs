use crate::emu;
use crate::winapi::helper;

pub fn BuildIoRingFlushFile(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!BuildIoRingFlushFile h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
