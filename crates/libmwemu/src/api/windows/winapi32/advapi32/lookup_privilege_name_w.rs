use crate::emu;

pub fn LookupPrivilegeNameW(emu: &mut emu::Emu) {
    let _system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let luid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let priv_name = "SeDebugPrivilege";
    if name_buf != 0 {
        emu.maps.write_wide_string(name_buf, priv_name);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, priv_name.len() as u32);
    }
    log_red!(
        emu,
        "advapi32!LookupPrivilegeNameW luid:0x{:x} => '{}'",
        luid_ptr,
        priv_name
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
