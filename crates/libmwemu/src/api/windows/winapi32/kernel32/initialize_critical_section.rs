use crate::emu;

pub fn InitializeCriticalSection(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!InitializeCriticalSection cannot read ptr_crit_sect");

    emu.stack_pop32(false);

    log_red!(
        emu,
        "kernel32!InitializeCriticalSection ptr: 0x{:x}",
        ptr_crit_sect
    );

    emu.regs_mut().rax = 1;
}
