use crate::emu;
use crate::winapi::helper;

pub fn HeapCreate(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let hndl = helper::handler_create("heap");
    log_red!(
        emu,
        "kernelbase!HeapCreate p1:0x{:x} p2:0x{:x} => 0x{:x}",
        p1,
        p2,
        hndl
    );
    emu.regs_mut().rax = hndl;
}
