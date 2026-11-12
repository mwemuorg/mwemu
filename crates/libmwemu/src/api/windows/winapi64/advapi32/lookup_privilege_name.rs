use crate::emu;
use crate::winapi::common::strings;

pub fn LookupPrivilegeNameA(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameA", false);
}

fn lookup_privilege_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _system_ptr = emu.regs().rcx;
    let luid_ptr = emu.regs().rdx;
    let name_ptr = emu.regs().r8;
    let name_sz_ptr = emu.regs().r9;
    let priv_name = "SeDebugPrivilege";
    log_red!(emu, "advapi32!{api} => '{}'", priv_name);
    if name_ptr != 0 {
        strings::write_string(emu, name_ptr, priv_name, wide);
    }
    if name_sz_ptr != 0 {
        emu.maps.write_dword(name_sz_ptr, priv_name.len() as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn LookupPrivilegeNameW(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameW", true);
}
