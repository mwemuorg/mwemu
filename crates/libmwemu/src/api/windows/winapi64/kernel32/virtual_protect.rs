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

    emu.regs_mut().rax = 1;
}
