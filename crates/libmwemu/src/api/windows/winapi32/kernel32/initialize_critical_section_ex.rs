use crate::emu;

pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!InitializeCriticalSectionEx cannot read ptr_crit_sect");
    let spin_count = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!InitializeCriticalSectionEx cannot read spin_count");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!InitializeCriticalSectionEx cannot read flags");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionEx ptr: 0x{:x}",
        ptr_crit_sect
    );

    emu.regs_mut().rax = 1;
}
