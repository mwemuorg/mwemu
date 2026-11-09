use crate::emu;
use crate::winapi::helper;

pub(super) fn crt_beginthread(emu: &mut emu::Emu) {
    let start = emu.regs().rcx;
    let stack_size = emu.regs().rdx;
    let arg = emu.regs().r8;
    let hndl = helper::handler_create(&format!("thread://0x{:x}", start));
    log_red!(
        emu,
        "wincrt!_beginthread start:0x{:x} stack:{} arg:0x{:x} => 0x{:x}",
        start,
        stack_size,
        arg,
        hndl
    );
    emu.regs_mut().rax = hndl;
}
