use crate::emu;

pub fn GetProcessesInVirtualizationContext(emu: &mut emu::Emu) {
    let pkg_family_ptr = emu.regs().rcx;
    let count_ptr = emu.regs().rdx;
    let processes_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetProcessesInVirtualizationContext pkg:0x{:x} count:0x{:x} procs:0x{:x}",
        pkg_family_ptr,
        count_ptr,
        processes_ptr
    );
    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
