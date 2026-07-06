use crate::emu;

pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;
    let flags = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionEx ptr: 0x{:x}",
        ptr_crit_sect
    );

    emu.regs_mut().rax = 1;
}
