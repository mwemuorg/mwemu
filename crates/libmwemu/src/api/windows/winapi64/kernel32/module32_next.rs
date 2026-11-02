use crate::emu;

pub fn Module32Next(emu: &mut emu::Emu) {
    let snap = emu.regs().rcx;
    let entry = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!Module32Next snap:0x{:x} entry:0x{:x}",
        snap,
        entry
    );
    emu.regs_mut().rax = 0; // FALSE
}
