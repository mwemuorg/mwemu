use crate::emu::Emu;

pub fn gateway(emu: &mut Emu) {
    let nr = emu.regs().rax;
    if !emu.call_syscall_hook(nr) {
        return;
    }
    let nr = emu.regs().rax as u32;
    log_orange!(emu, "syscall 0x{:x}: (unimplemented)", nr);
}
