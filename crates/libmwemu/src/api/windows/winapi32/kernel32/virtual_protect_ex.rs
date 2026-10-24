use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    let hproc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let addr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let new_prot = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let old_prot_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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
