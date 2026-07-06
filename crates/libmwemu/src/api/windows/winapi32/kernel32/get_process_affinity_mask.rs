use crate::emu;

pub fn GetProcessAffinityMask(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetProcessAffinityMask cannot read the handle") as u64;
    let proc_affinity_mask_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetProcessAffinityMask cannot read the  proc_affinity_mask_ptr")
        as u64;
    let sys_affinity_mask_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!GetProcessAffinityMask cannot read the sys_affinity_mask_ptr")
        as u64;

    emu.maps.write_dword(proc_affinity_mask_ptr, 0x1337);
    emu.maps.write_dword(sys_affinity_mask_ptr, 0x1337);

    log_red!(emu, "kernel32!GetProcessAffinityMask");

    emu.regs_mut().rax = 1;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
