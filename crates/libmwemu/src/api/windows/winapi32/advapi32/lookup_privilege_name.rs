use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn LookupPrivilegeNameA(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameA", false);
}

fn lookup_privilege_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _system_ptr = WinApiAbi::arg(emu, 0);
    let luid_ptr = WinApiAbi::arg(emu, 1);
    let name_buf = WinApiAbi::arg(emu, 2);
    let size_ptr = WinApiAbi::arg(emu, 3);
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
