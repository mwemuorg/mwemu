use crate::emu;

pub fn VirtualProtect(emu: &mut emu::Emu) {
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let new_prot = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let old_prot_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

    emu.maps.write_dword(old_prot_ptr, new_prot);

    log_red!(
        emu,
        "kernel32!VirtualProtect addr: 0x{:x} sz: {} prot: {}",
        addr,
        size,
        new_prot
    );

    // When making memory executable, check if the region contains a PE with TLS
    let is_exec = (new_prot & 0xF0) != 0; // PAGE_EXECUTE(0x10)|READ(0x20)|RW(0x40)|WC(0x80)
    if is_exec {
        let base = emu.maps.get_addr_base(addr).unwrap_or(addr);
        emu.init_tls_from_pe32_in_memory(base);
    }

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
