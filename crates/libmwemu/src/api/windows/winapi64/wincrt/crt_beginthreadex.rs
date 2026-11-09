use crate::emu;
use crate::winapi::helper;

pub(super) fn crt_beginthreadex(emu: &mut emu::Emu) {
    let _security = emu.regs().rcx;
    let _stack_size = emu.regs().rdx;
    let start = emu.regs().r8;
    let arg = emu.regs().r9;
    let hndl = helper::handler_create(&format!("thread://0x{:x}", start));
    log_red!(
        emu,
        "wincrt!_beginthreadex start:0x{:x} arg:0x{:x} => 0x{:x}",
        start,
        arg,
        hndl
    );
    emu.regs_mut().rax = hndl;
}
