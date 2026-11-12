use crate::emu;
use crate::winapi::common::strings;

pub fn LookupPrivilegeNameA(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameA", false);
}

fn lookup_privilege_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let luid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let priv_name = "SeDebugPrivilege";
    if name_buf != 0 {
        strings::write_string(emu, name_buf, priv_name, wide);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, priv_name.len() as u32);
    }
    log_red!(
        emu,
        "advapi32!{api} luid:0x{:x} => '{}'",
        luid_ptr,
        priv_name
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn LookupPrivilegeNameW(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameW", true);
}
