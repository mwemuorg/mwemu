use crate::emu;

pub fn LookupPrivilegeValueA(emu: &mut emu::Emu) {
    let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let luid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let name = emu.maps.read_string(name_ptr);
    if luid_ptr != 0 {
        emu.maps.write_dword(luid_ptr, 20);
        emu.maps.write_dword(luid_ptr + 4, 0);
    }
    log_red!(emu, "advapi32!LookupPrivilegeValueA name:'{}'", name);
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
