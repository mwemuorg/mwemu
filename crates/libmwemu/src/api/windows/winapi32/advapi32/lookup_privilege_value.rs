use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn LookupPrivilegeValueA(emu: &mut emu::Emu) {
    lookup_privilege_value(emu, "LookupPrivilegeValueA", false);
}

pub(super) fn LookupPrivilegeValueW(emu: &mut emu::Emu) {
    lookup_privilege_value(emu, "LookupPrivilegeValueW", true);
}

fn lookup_privilege_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    if wide {
        let ptr_sysname = WinApiAbi::arg(emu, 0);
        let ptr_name = WinApiAbi::arg(emu, 1);
        let ptr_uid = WinApiAbi::arg(emu, 2);
        let sysname = strings::read_text(emu, ptr_sysname, wide);
        let name = strings::read_text(emu, ptr_name, wide);
        log_red!(emu, "advapi32!{api} `{}` `{}`", sysname, name);
        if ptr_uid != 0 {
            let luid: u64 = match name.as_str() {
                "SeDebugPrivilege" => 20,
                "SeShutdownPrivilege" => 19,
                "SeBackupPrivilege" => 17,
                "SeRestorePrivilege" => 18,
                "SeTcbPrivilege" => 7,
                "SeAssignPrimaryTokenPrivilege" => 3,
                "SeIncreaseQuotaPrivilege" => 5,
                _ => 99,
            };
            emu.maps.write_qword(ptr_uid, luid);
        }
    } else {
        let system_ptr = WinApiAbi::arg(emu, 0);
        let name_ptr = WinApiAbi::arg(emu, 1);
        let luid_ptr = WinApiAbi::arg(emu, 2);
        let name = strings::read_text(emu, name_ptr, wide);
        if luid_ptr != 0 {
            emu.maps.write_dword(luid_ptr, 20);
            emu.maps.write_dword(luid_ptr + 4, 0);
        }
        log_red!(emu, "advapi32!{api} name:'{}'", name);
    }

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
