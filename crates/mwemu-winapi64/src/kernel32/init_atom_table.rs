use crate::emu;
use crate::winapi::helper;
pub fn InitAtomTable(emu: &mut emu::Emu) {
    let n_size = emu.regs().rcx;
    log_red!(emu, "kernel32!InitAtomTable size:{}", n_size);
    let handle = helper::handler_create("InitAtomTable");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
