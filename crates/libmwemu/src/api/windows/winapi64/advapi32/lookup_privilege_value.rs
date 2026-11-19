use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn LookupPrivilegeValueA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let system_ptr = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let luid_ptr = api_try!(emu, call.arg(emu, 2));
    let name = emu.maps.read_string(name_ptr);
    if luid_ptr != 0 {
        emu.maps.write_dword(luid_ptr, 20);
        emu.maps.write_dword(luid_ptr + 4, 0);
    }
    log_red!(emu, "advapi32!LookupPrivilegeValueA name:'{}'", name);
    call.set_return(emu, 1);
}

pub(super) fn LookupPrivilegeValueW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr_sysname = api_try!(emu, call.arg(emu, 0));
    let ptr_name = api_try!(emu, call.arg(emu, 1));
    let ptr_uid = api_try!(emu, call.arg(emu, 2));

    let sysname = emu.maps.read_wide_string(ptr_sysname);
    let name = emu.maps.read_wide_string(ptr_name);

    log_red!(
        emu,
        "advapi32!LookupPrivilegeValueW `{}` `{}`",
        sysname,
        name
    );

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

    call.set_return(emu, 1);
}
