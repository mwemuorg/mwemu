use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    let hproc = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!VirtualProtectEx cannot read hproc") as u64;
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!VirtualProtectEx cannot read addr") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!VirtualProtectEx cannot read size");
    let new_prot = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!VirtualProtectEx cannot read new_prot");
    let old_prot_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!VirtualProtectEx cannot read old_prot") as u64;

    emu.maps.write_dword(old_prot_ptr, new_prot);

    log_red!(
        emu,
        "kernel32!VirtualProtectEx hproc: {} addr: 0x{:x} sz: {} prot: {}",
        hproc,
        addr,
        size,
        new_prot
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
