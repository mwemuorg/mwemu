use crate::emu;
use crate::winapi::helper;

pub fn HeapCreate(emu: &mut emu::Emu) {
    let opts = emu.regs().rcx;
    let initSZ = emu.regs().rdx;
    let maxSZ = emu.regs().r8;

    log_red!(emu, "kernel32!HeapCreate maxSZ:{}", maxSZ);

    let uri = format!("HeapCreate://{}", maxSZ);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
