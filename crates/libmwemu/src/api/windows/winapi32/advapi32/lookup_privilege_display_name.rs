use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn LookupPrivilegeDisplayNameA(emu: &mut emu::Emu) {
    lookup_privilege_display_name(emu, "LookupPrivilegeDisplayNameA", false);
}

fn lookup_privilege_display_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn LookupPrivilegeDisplayNameW(emu: &mut emu::Emu) {
    lookup_privilege_display_name(emu, "LookupPrivilegeDisplayNameW", true);
}
