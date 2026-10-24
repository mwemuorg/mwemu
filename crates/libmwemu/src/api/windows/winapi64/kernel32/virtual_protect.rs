use crate::emu;

pub fn VirtualProtect(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let new_prot = emu.regs().r8;
    let old_prot_ptr = emu.regs().r9;

    emu.maps.write_qword(old_prot_ptr, new_prot);

    log_red!(
        emu,
        "kernel32!VirtualProtect addr: 0x{:x} sz: {} prot: {}",
        addr,
        size,
        new_prot
    );

    let is_exec = (new_prot & 0xF0) != 0;
    if is_exec {
        let base = emu.maps.get_addr_base(addr).unwrap_or(addr);
        emu.init_tls_from_pe64_in_memory(base);
    }

    emu.regs_mut().rax = 1;
}
