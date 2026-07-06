use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    let hproc = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let size = emu.regs().r8;
    let new_prot = emu.regs().r9;
    let oldld_prot_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!VirtualProtectEx cannot read old_prot");

    log_red!(
        emu,
        "kernel32!VirtualProtectEx hproc: {} addr: 0x{:x} sz: {} prot: {}",
        hproc,
        addr,
        size,
        new_prot
    );

    emu.regs_mut().rax = 1;
}
