use crate::emu;

pub fn GetSystemRegistryQuota(emu: &mut emu::Emu) {
    let allowed_ptr = emu.regs().rcx;
    let used_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetSystemRegistryQuota allowed:0x{:x} used:0x{:x}",
        allowed_ptr,
        used_ptr
    );
    if allowed_ptr != 0 {
        emu.maps.write_dword(allowed_ptr, 0x2000000);
    }
    if used_ptr != 0 {
        emu.maps.write_dword(used_ptr, 0x100000);
    }
    emu.regs_mut().rax = 1;
}
