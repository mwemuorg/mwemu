use crate::emu;

pub fn LookupPrivilegeValueA(emu: &mut emu::Emu) {
    let system_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let luid_ptr = emu.regs().r8;
    let name = emu.maps.read_string(name_ptr);
    if luid_ptr != 0 {
        emu.maps.write_dword(luid_ptr, 20);
        emu.maps.write_dword(luid_ptr + 4, 0);
    }
    log_red!(emu, "advapi32!LookupPrivilegeValueA name:'{}'", name);
    emu.regs_mut().rax = 1;
}
