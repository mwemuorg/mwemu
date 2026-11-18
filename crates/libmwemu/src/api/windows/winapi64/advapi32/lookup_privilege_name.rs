use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn LookupPrivilegeNameA(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameA", false);
}

fn lookup_privilege_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _system_ptr = api_try!(emu, call.arg(emu, 0));
    let luid_ptr = api_try!(emu, call.arg(emu, 1));
    let name_ptr = api_try!(emu, call.arg(emu, 2));
    let name_sz_ptr = api_try!(emu, call.arg(emu, 3));
    let priv_name = "SeDebugPrivilege";
    log_red!(emu, "advapi32!{api} => '{}'", priv_name);
    if name_ptr != 0 {
        strings::write_string(emu, name_ptr, priv_name, wide);
    }
    if name_sz_ptr != 0 {
        emu.maps.write_dword(name_sz_ptr, priv_name.len() as u32);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn LookupPrivilegeNameW(emu: &mut emu::Emu) {
    lookup_privilege_name(emu, "LookupPrivilegeNameW", true);
}
