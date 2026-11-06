use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    let hproc = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let size = emu.regs().r8;
    let new_prot = emu.regs().r9 as u32;
    let old_prot_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    if old_prot_ptr != 0 {
        emu.maps.write_dword(old_prot_ptr, new_prot);
    }

    log_red!(
        emu,
        "kernel32!VirtualProtectEx hproc: {} addr: 0x{:x} sz: {} prot: {}",
        hproc,
        addr,
        size,
        new_prot
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
