use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn LookupPrivilegeDisplayNameA(emu: &mut emu::Emu) {
    lookup_privilege_display_name(emu, "LookupPrivilegeDisplayNameA", false);
}

fn lookup_privilege_display_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let system_ptr = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn LookupPrivilegeDisplayNameW(emu: &mut emu::Emu) {
    lookup_privilege_display_name(emu, "LookupPrivilegeDisplayNameW", true);
}
