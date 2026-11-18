use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn LookupPrivilegeValueA(emu: &mut emu::Emu) {
    lookup_privilege_value(emu, "LookupPrivilegeValueA", false);
}

pub(super) fn LookupPrivilegeValueW(emu: &mut emu::Emu) {
    lookup_privilege_value(emu, "LookupPrivilegeValueW", true);
}

fn lookup_privilege_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    if wide {
        let ptr_sysname = api_try!(emu, call.arg(emu, 0));
        let ptr_name = api_try!(emu, call.arg(emu, 1));
        let ptr_uid = api_try!(emu, call.arg(emu, 2));
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
        let system_ptr = api_try!(emu, call.arg(emu, 0));
        let name_ptr = api_try!(emu, call.arg(emu, 1));
        let luid_ptr = api_try!(emu, call.arg(emu, 2));
        let name = strings::read_text(emu, name_ptr, wide);
        if luid_ptr != 0 {
            emu.maps.write_dword(luid_ptr, 20);
            emu.maps.write_dword(luid_ptr + 4, 0);
        }
        log_red!(emu, "advapi32!{api} name:'{}'", name);
    }

    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}
