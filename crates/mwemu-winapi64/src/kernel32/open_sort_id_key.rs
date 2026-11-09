use crate::emu;
use crate::winapi::helper;

pub fn OpenSortIdKey(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenSortIdKey");

    emu.regs_mut().rax = helper::handler_create("OpenSortIdKey");
}
