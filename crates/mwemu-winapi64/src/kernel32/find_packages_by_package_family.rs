use crate::emu;

pub fn FindPackagesByPackageFamily(emu: &mut emu::Emu) {
    let family_name_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let count_ptr = emu.regs().r8;
    let names_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!FindPackagesByPackageFamily name:0x{:x} flags:0x{:x}",
        family_name_ptr,
        flags
    );
    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
