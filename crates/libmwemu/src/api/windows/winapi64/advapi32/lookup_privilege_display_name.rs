use crate::emu;
use crate::winapi::common::strings;

pub fn LookupPrivilegeDisplayNameA(emu: &mut emu::Emu) {
    lookup_privilege_display_name(emu, "LookupPrivilegeDisplayNameA", false);
}

fn lookup_privilege_display_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let system_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn LookupPrivilegeDisplayNameW(emu: &mut emu::Emu) {
    lookup_privilege_display_name(emu, "LookupPrivilegeDisplayNameW", true);
}
