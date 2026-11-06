use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    let hproc = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let size = emu.regs().r8;
    let new_prot = emu.regs().r9 as u32;
    let old_prot_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!VirtualProtectEx hproc: {} addr: 0x{:x} sz: {} prot: {}",
        hproc,
        addr,
        size,
        new_prot
    );

    super::protect(emu, addr, size, new_prot, old_prot_ptr);
}
