use crate::emu;
use crate::winapi::helper;
pub fn FlsAlloc(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;
    let idx = emu.fls().len() as u64;
    emu.fls_mut().push(0);
    log_red!(
        emu,
        "kernel32!FlsAlloc callback:0x{:x} => idx:{}",
        callback,
        idx
    );
    let handle = helper::handler_create("FlsAlloc");
    emu.set_last_error(0);
    emu.regs_mut().rax = idx;
}
