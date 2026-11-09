use crate::emu;
pub fn HeapValidate(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    log_red!(emu, "kernel32!HeapValidate p1:0x{:x} p2:0x{:x}", p1, p2);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
