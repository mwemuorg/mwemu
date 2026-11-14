use crate::emu;
use crate::winapi::helper;

pub fn ReleaseActCtx(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(emu, "kernelbase!ReleaseActCtx handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
}
