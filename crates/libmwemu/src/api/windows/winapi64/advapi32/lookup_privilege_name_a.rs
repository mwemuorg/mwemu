use crate::emu;

pub fn LookupPrivilegeNameA(emu: &mut emu::Emu) {
    let _system_ptr = emu.regs().rcx;
    let luid_ptr = emu.regs().rdx;
    let name_ptr = emu.regs().r8;
    let name_sz_ptr = emu.regs().r9;
    let priv_name = "SeDebugPrivilege";
    log_red!(emu, "advapi32!LookupPrivilegeNameA => '{}'", priv_name);
    if name_ptr != 0 {
        emu.maps.write_string(name_ptr, priv_name);
    }
    if name_sz_ptr != 0 {
        emu.maps.write_dword(name_sz_ptr, priv_name.len() as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
