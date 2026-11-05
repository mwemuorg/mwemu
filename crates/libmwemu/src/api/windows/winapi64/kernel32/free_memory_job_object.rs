use crate::emu;

pub fn FreeMemoryJobObject(emu: &mut emu::Emu) {
    let _Buffer = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!FreeMemoryJobObject");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
