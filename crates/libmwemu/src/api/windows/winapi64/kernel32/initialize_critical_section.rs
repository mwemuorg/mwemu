use crate::emu;

pub fn InitializeCriticalSection(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!InitializeCriticalSection ptr: 0x{:x}",
        ptr_crit_sect
    );

    emu.regs_mut().rax = 1;
}
